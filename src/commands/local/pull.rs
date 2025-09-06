use std::path::PathBuf;

use clap::{arg, Arg, ArgMatches, Command, command};
use color_print::cformat;

use crate::commands::core::args::Arguments;
use crate::girep::error::structs::Error;
use crate::girep::platform::Platform;
use crate::local::git::options::{Methods, Options};
use crate::local::git::structs::Action;
use crate::local::pull::PullAction;
use crate::usettings::structs::Usettings;
use crate::animations::animation::Fetch;
use crate::girep::animation::Animation;


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
                .conflicts_with("set-upstream"),
            arg!([branch] "The name of the branch to pull from")
                .conflicts_with("set-upstream"),
            Arguments::path_flag(false, "Path to the repository"),
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Fetching the repository ...");
    
    let pconf = match args.get_one::<String>("pconf") {
        Some(e) if e == "-" => Some(usettings.get_default_pconf().unwrap()),
        Some(e) => Some(usettings.get_pconf_by_name(e).unwrap()),
        None => None,
    };
    
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
    
    let result = Platform::pull_repo(&path, options, pconf.clone(), action, &usettings, Some(&animation));
    
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
            animation.finish_with_warning(cformat!("<m,i>push</m,i> <y>finish with errors!</>"));
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