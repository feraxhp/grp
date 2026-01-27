use std::path::PathBuf;

use clap::{arg, command, Arg, ArgMatches, Command};
use color_print::cformat;
use grp_core::animation::Animation;
use grp_core::Error;

use crate::commands::completions::git::branch::Branch;
use crate::commands::completions::git::remote::Remote;
use crate::commands::completions::structure::Completer;
use crate::commands::core::args::Arguments;
use crate::local::structs::{Git2Error, Local};
use crate::usettings::structs::{Pconf, Usettings};
use crate::local::git::{structs::Action};
use crate::local::git::options::{Methods, Options};
use crate::animations::animation::Create;

pub(crate) fn command() -> Command {
    command!("push").aliases(["p"])
        .about(cformat!("Interface to <b,i>git push</> using the given pconf"))
        .args([
            Arguments::pconf(false, true),
            arg!( -A --all "Push all branches")
                .conflicts_with_all(["branches", "tags", "set-upstream", "branch", "tag"])
            ,
            arg!( -B --branches "Push all branches")
                .conflicts_with_all(["all", "tags", "set-upstream", "branch", "tag"])
            ,
            arg!( -T --tags "Push all tags")
                .conflicts_with_all(["all", "branches", "set-upstream", "branch", "tag"])
            ,
            arg!( -t --tag [tag] "Push te given tag")
                .require_equals(true)
                .conflicts_with_all(["all", "branches", "set-upstream", "branch"])
            ,
            arg!( -f --force "Do a force push"),
            arg!( -n --"dry-run" "Do everything except actually send the updates."),
            Arg::new("set-upstream").short('u').long("set-upstream")
                .num_args(2)
                .value_names(["remote", "branch"])
                .conflicts_with_all(["all", "branches", "tags", "tag"])
                .help("Sets the name of the remote as default upstream for a branch"),
            arg!([remote] "The name of the remote to push to")
                .conflicts_with("set-upstream")
                .add(Remote::complete())
            ,
            arg!([branch] "The name of the branch to push")
                .conflicts_with_all(["set-upstream", "tag"])
                .add(Branch::complete())
            ,
            Arguments::path_flag(false, "Path to the repository"),
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let mut animation = Create::new("Preparing to push ...");

    let pconf = args.get_one::<Pconf>("pconf").map(| p| p.clone());
    
    let (tag, tag_) = match args.get_one::<String>("tag") {
        Some(t) => (t.clone(), true),
        None => ("".to_string(), false),
    };
    
    
    let all = args.get_flag("all");
    let branches = args.get_flag("branches");
    let tags = args.get_flag("tags");
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
    
    let method = match "" {
        _ if all => Methods::ALL,
        _ if branches => Methods::BRANCHES,
        _ if tags => Methods::TAGS,
        _ if up_stream => Methods::UPSTREAM,
        _ if tag_ => Methods::TAG(tag),
        _ => Methods::DEFAULT
    };
    
    let options = Options {
        method, remote,
        branch, force,
        dry_run
    };

    let result = Local::push_repo(&path, pconf.clone(), options, &usettings, &mut animation);
    
    let logs = match result {
        Ok((logs, true)) => {
            let message = match "" {
                _ if dry_run => cformat!("<y,i>dry-run</y,i> <g>finish without errors</>"),
                _ => cformat!("<y,i>push</y,i> <g>succeeded!</>")
            };
            
            animation.finish_with_success(message);
            logs
        },
        Ok((logs, false)) => {
            animation.finish_with_warning(cformat!("<m,i>push</m,i> <y>finish with errors!</>"));
            logs
        },
        Err(e) => {
            let action =  Action::Push;
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