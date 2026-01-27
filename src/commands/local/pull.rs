use std::path::PathBuf;

use clap::{arg, Arg, ArgMatches, Command, command};
use color_print::cformat;

use grp_core::animation::Animation;
use grp_core::Error;

use crate::commands::completions::git::branch::Branch;
use crate::commands::completions::git::remote::Remote;
use crate::commands::completions::structure::Completer;
use crate::commands::core::args::Arguments;
use crate::local::git::options::{Methods, Options};
use crate::local::git::structs::Action;
use crate::local::pull::PullAction;
use crate::local::structs::{Git2Error, Local};
use crate::usettings::structs::{Pconf, Usettings};
use crate::animations::animation::Fetch;


pub(crate) fn command() -> Command {
    command!("pull").aliases(["j"])
        .about(cformat!("Interface to <b,i>git pull</> using the given pconf"))
        .args([
            Arguments::pconf(false, true),
            arg!( -f --force "Do a force pull"),
            arg!( -r --rebase).help(cformat!("Do a <i>pull --rebase</>")),
            arg!( -n --"dry-run" "Do everything except actually fetch the updates."),
            Arg::new("set-upstream").short('u').long("set-upstream")
                .num_args(2)
                .value_names(["remote", "branch"])
                .help("Sets the name of the remote as default upstream for a branch"),
            arg!([remote] "The name of the remote to pull from")
                .conflicts_with("set-upstream")
                .add(Remote::complete()),
            arg!([branch] "The name of the branch to pull from")
                .conflicts_with("set-upstream")
                .add(Branch::complete()),
            Arguments::path_flag(false, "Path to the repository"),
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let mut animation = Fetch::new("Fetching the repository ...");
    
    let pconf = args.get_one::<Pconf>("pconf").map(| p| p.clone());
    
    let force = args.get_flag("force");
    let dry_run = args.get_flag("dry-run");
    
    let path = match args.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap()
    };
    
    let mut remote = match args.get_one::<String>("remote") {
        None => None,
        Some(e) => Some(e.to_owned())
    };

    let mut branch = match args.get_one::<String>("branch") {
        None => None,
        Some(e) => Some(e.to_owned())
    };
    
    let up_stream = if let Some(values) = args.get_many::<String>("set-upstream") {
        let mut values_iter = values.clone();
        remote = Some(values_iter.next().unwrap().to_owned());
        branch = Some(values_iter.next().unwrap().to_owned());
        true
    } else { false };
    
    let options = Options {
        method: if up_stream { Methods::UPSTREAM } else { Methods::DEFAULT },
        remote, branch,
        force, dry_run
    };
    
    let action = match args.get_flag("rebase") {
        true => PullAction::REBASE,
        false => PullAction::MERGE,
    };
    
    let result = Local::pull_repo(&path, options, pconf.clone(), action, &usettings, &mut animation);
    
    let logs = match result {
        Ok((logs, true)) => {
            let message = match "" {
                _ if dry_run => cformat!("<y,i>dry-run</y,i> <g>finish without errors</>"),
                _ => cformat!("<y,i>pull</y,i> <g>succeeded!</>")
            };
            
            animation.finish_with_success(message);
            logs
        },
        Ok((logs, false)) => {
            animation.finish_with_warning(cformat!("<m,i>pull</m,i> <y>finish with errors!</>"));
            logs
        },
        Err(e) => {
            let action =  Action::Pull;
            let path = path.as_os_str().to_str().unwrap_or("{{ Break path }}");
            
            let error = match pconf {
                Some(p) => {
                    let config = p.to_config();
                    Error::from_git2(e, action, &path, Some(&config))
                }
                None => Error::from_git2(e, action, &path, None),
            };
            
            animation.finish_with_error(&error.message);
            error.show();
            return;
        },
    };
    
    for log in logs { println!("{}", log); }
}