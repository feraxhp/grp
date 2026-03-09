use std::process::exit;
use futures::StreamExt;

use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};
use grp_core::animation::Animation;
use grp_core::{Error, Platform};

use crate::animations::animation::Fetch;
use crate::cache::structure::Cacher;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::system::show::Show;
use crate::usettings::structs::{Pconf, Usettings};

pub fn command() -> Command {
    Commands::list("List the all the repositories from a configured repo and the given user")
        .args([
            Arguments::pconf(false, true),
            arg!([owner] "The user to list the repositories from"),
            arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing list repositories");
    
    let pconf = match args.get_one::<Pconf>("pconf") {
        Some(e) => e.clone(),
        None => usettings.get_default_pconf().or_exit(&animation),
    };
    
    let owner = args.get_one::<String>("owner");
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
    
    let stream = match platform.list_repos(owner, &config, &animation).await {
        Ok(s) => s,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            return;
        },
    };
    
    let an = &animation;
    let (repos, mut errors) = stream
        .enumerate()
        .map(|(i, s)| {
            an.change_message(format!("Requesting page: {}", i + 1));
            s
        })
        .fold((vec![], vec![]), async move |curr, act| {
            let (mut repos, mut errors) = curr;
            match act {
                Ok(r) => repos.extend(r),
                Err(e) => errors.push(e),
            }
            (repos, errors)
        })
        .await;
    
    match repos.save(&pconf.name, !owner.is_none()) {
        Ok(_) => (),
        Err(e) => errors.push(e),
    };
    
    match (repos.is_empty(), errors.is_empty()) {
        (true, true) => { animation.finish_with_success("<i>No repos found</>"); },
        (false,  true) => {
            animation.finish_with_success(cformat!("<y,i>list repos</y,i> <g>succeeded!</>"));
            repos.print_pretty();
        },
        (true, false) => {
            let error = Error::collection(errors);
            animation.finish_with_error(format!("{}", error.message));
            error.show();
        },
        (false, false) => {
            animation.finish_with_warning(cformat!("<m,i>list repos</m,i> <y>finish with errors!</>"));
            repos.print_pretty();
            if show_errors { errors.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
    }
}
