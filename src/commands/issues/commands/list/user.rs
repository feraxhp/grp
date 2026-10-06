use std::process::exit;
use futures::StreamExt;

use clap::{Arg, ArgMatches, arg};
use color_print::{cformat, cprintln};
use grp_core::animation::Animation;
use grp_core::{Error, Platform};

use crate::animations::animation::Fetch;
use crate::commands::core::args::Arguments;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::printables::show::Show;
use crate::usettings::structs::{Pconf, Usettings};


pub fn arguments() -> [Arg; 4] {
    [
        Arguments::pconf(false, true),
        arg!(-a --assigned "Show only the assigned issues for the authenticated user")
        ,
        arg!(-i --involved "Show all the involved issues for the authenticated user (only: github)")
            .conflicts_with_all(["assigned"])
        ,
        arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
            .required(false)
    ]
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing list issues");
    let pconf = match args.get_one::<Pconf>("pconf") {
        Some(e) => e.clone(),
        None => usettings.get_default_pconf().or_exit(&animation),
    };

    let mode = match true {
        _ if args.get_flag("involved") => Mode::INVOLVED,
        _ if args.get_flag("assigned") => Mode::ASSIGNED,
        _ => Mode::ALL,
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
    
    let stream = match mode {
        Mode::ASSIGNED => match platform.list_user_assigned_issues(&config, &animation).await {
            Ok(s) => s.boxed(),
            Err(e) => {
                animation.finish_with_error(&e.message);
                e.show();
                return;
            },
        },
        Mode::ALL => match platform.list_all_user_issues(&config, &animation).await {
            Ok(s) => s.boxed(),
            Err(e) => {
                animation.finish_with_error(&e.message);
                e.show();
                return;
            },
        },
        Mode::INVOLVED => match platform.list_all_involved_user_issues(&config, &animation).await {
            Ok(s) => s.boxed(),
            Err(e) => {
                animation.finish_with_error(&e.message);
                e.show();
                return;
            },
        },
    };
    
    let an = &animation;
    let (repos, errors) = stream
        .enumerate()
        .take(1)
        .map(|(i, s)| {
            match &s {
            Ok(pr) if let Some(last) = pr.pager.last_number() => {
                    an.change_message(format!("Requesting page: {} of {}", i + 1, &last))
                },
                _ => an.change_message(format!("Requesting page: {}", i + 1)),
            }
            s
        })
        .fold((vec![], vec![]), async move |curr, act| {
            let (mut repos, mut errors) = curr;
            match act {
                Ok(r) => repos.extend(r.result),
                Err(e) => errors.push(e),
            }
            (repos, errors)
        })
        .await;
    
    match (repos.is_empty(), errors.is_empty()) {
        (true, true) => { animation.finish_with_success("<i>No issues found</>"); },
        (false,  true) => {
            animation.finish_with_success(cformat!("<y,i>list issues</y,i> <g>succeeded!</>"));
            repos.print_pretty();
        },
        (true, false) => {
            let error = Error::collection(errors);
            animation.finish_with_error(format!("{}", error.message));
            error.show();
        },
        (false, false) => {
            animation.finish_with_warning(cformat!("<m,i>list issues</m,i> <y>finish with errors!</>"));
            repos.print_pretty();
            if show_errors { errors.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
    }
}

enum Mode {
    ALL,
    ASSIGNED,
    INVOLVED,
}
