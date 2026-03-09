use std::vec;
use std::process::exit;
use futures::StreamExt;

use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};
use grp_core::animation::Animation;
use grp_core::{Error, Platform};

use crate::cache::structure::Cacher;
use crate::system::show::Show;
use crate::animations::animation::Fetch;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::usettings::structs::{Pconf, Usettings};


pub fn command() -> Command {
    Commands::list("List the organizations for the authenticated user")
        .args([
            Arguments::pconf(false, true),
            arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing list organizations");
    
    let pconf = match args.get_one::<Pconf>("pconf") {
        Some(e) => e.clone(),
        None => usettings.get_default_pconf().or_exit(&animation),
    };
    
    let show_errors = args.get_flag("show-errors");
    
    let platform = match Platform::matches(&pconf.r#type) {
        Ok(p) => p,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1)
        },
    };
    let config = pconf.to_config();
    
    let an = &animation;
    let (orgs, mut errors) = platform.list_orgs(&config, &animation)
        .enumerate()
        .map(|(i, s)| {
            an.change_message(format!("Requesting page: {}", i + 1));
            s
        })
        .fold((vec![], vec![]), async move |acc, act| {
            let (mut users, mut errors) = acc;
            match act {
                Ok(u) => users.extend(u),
                Err(e) => errors.push(e),
            }
            (users, errors)
        })
        .await
    ;
    
    match orgs.save(&pconf.name, false) {
        Ok(_) => (),
        Err(e) => errors.push(e),
    };
    
    match (orgs.is_empty(), errors.is_empty()) {
        (true, true) => {
            animation.finish_with_success("<i>No orgs found</>");
        },
        (_, true) => {
            animation.finish_with_success(cformat!("<y,i>list orgs</y,i> <g>succeeded!</>"));
            orgs.print_pretty();
        },
        (true, _) => {
            let error = Error::collection(errors);
            animation.finish_with_error(format!("{}", error.message));
            error.show();
        },
        (false, false) => {
            animation.finish_with_warning(cformat!("<m,i>list orgs</m,i> <y>finish with errors!</>"));
            orgs.print_pretty();
            if show_errors { errors.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
    }
}