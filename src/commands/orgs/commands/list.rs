use std::vec;
use std::process::exit;
use futures::StreamExt;

use clap::{arg, ArgMatches, Command};
use color_print::cprintln;
use grp_core::animation::Animation;
use grp_core::Platform;
use mynox::OptionError;

use crate::cache::structure::Cacher;
use crate::{success, warning};
use crate::printables::show::Show;
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
        None => usettings.get_default_pconf(&animation).or_exit(&animation),
    };
    
    let show_errors = args.get_flag("show-errors");
    
    let platform = match Platform::matches(&pconf.r#type) {
        Ok(p) => p,
        Err(e) => {
            animation.finish_with_error(e.message());
            e.eprint();
            exit(1)
        },
    };
    let config = pconf.to_config();
    
    let an = &animation;
    let (orgs, mut errors) = platform.list_orgs(&config, &animation)
        .enumerate()
        .map(|(i, s)| {
            match &s {
                Ok(pr) if let Some(last) = pr.pager.last_number() => {
                    an.change_message(format!("Requesting page: {} of {}", i + 1, last))
                },
                _ => an.change_message(format!("Requesting page: {}", i + 1)),
            }
            s
        })
        .fold((vec![], vec![]), async move |acc, act| {
            let (mut users, mut errors) = acc;
            match act {
                Ok(u) => users.extend(u.result),
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
    
    let errors = errors.into_iter().collect::<OptionError>().into_option();
    match (orgs.is_empty(), errors.is_none()) {
        (true, true) => {
            animation.finish_with_success("<i>No orgs found</>");
        },
        (_, true) => {
            animation.finish_with_success(success!(np "list orgs"));
            orgs.print_pretty();
        },
        (true, _) => {
            let error = errors.unwrap();
            animation.finish_with_error(error.message());
            error.print();
        },
        (false, false) => {
            animation.finish_with_warning(warning!(np "list orgs"));
            orgs.print_pretty();
            if show_errors { errors.unwrap().eprint(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
    }
}
