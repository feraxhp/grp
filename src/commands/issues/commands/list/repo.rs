use std::process::exit;
use futures::StreamExt;

use clap::{arg, ArgMatches, Command, command};
use color_print::{cformat, cprintln};
use grp_core::animation::Animation;
use grp_core::Platform;
use mynox::OptionError;

use crate::animations::animation::Fetch;
use crate::commands::core::args::Arguments;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::commands::validations::repo::RepoStructure;
use crate::printables::show::Show;
use crate::usettings::structs::Usettings;

pub fn command() -> Command {
    command!()
        .name("repo")
        .about("list the issues from a repository")
        .args([
            Arguments::repo_structure(false, true),
            arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing list issues");

    let repo = args.get_one::<RepoStructure> ("repo").unwrap();
    let pconf = match repo.pconf.clone() {
        Some(e) => usettings.get_pconf_by_name(e.as_str(), &animation).unwrap(),
        None => usettings.get_default_pconf(&animation).or_exit(&animation),
    };
    
    let owner = Some(&repo.owner);
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
    
    let stream = match platform.list_issues(owner, &repo.path, &config, &animation).await {
        Ok(s) => s,
        Err(e) => {
            animation.finish_with_error(e.message());
            e.eprint();
            return;
        },
    };
    
    let an = &animation;
    let (repos, errors) = stream
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
        .fold((vec![], vec![]), async move |curr, act| {
            let (mut repos, mut errors) = curr;
            match act {
                Ok(r) => repos.extend(r.result),
                Err(e) => errors.push(e),
            }
            (repos, errors)
        })
        .await;
    
    let errors = errors.into_iter().collect::<OptionError>().into_option();
    match (repos.is_empty(), errors.is_none()) {
        (true, true) => { animation.finish_with_success("<i>No issues found</>"); },
        (false,  true) => {
            animation.finish_with_success(cformat!("<y,i>list issues</y,i> <g>succeeded!</>"));
            repos.print_pretty();
        },
        (true, false) => {
            let error = errors.unwrap();
            animation.finish_with_error(error.message().to_string());
            error.eprint();
            exit(1)
        },
        (false, false) => {
            animation.finish_with_warning(cformat!("<m,i>list issues</m,i> <y>finish with errors!</>"));
            repos.print_pretty();
            if show_errors { errors.unwrap().eprint(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
    }
}
