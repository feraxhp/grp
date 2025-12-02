use std::path::PathBuf;
use clap::{ArgMatches, Command, arg, command};
use color_print::cformat;

use super::super::completions::structure::Completer;

use crate::commands::completions::git::remote::Remote;
use crate::girep::usettings::structs::{Pconf, Usettings};
use crate::local::git::options::{Methods, Options};
use crate::local::git::structs::Action;
use crate::girep::{animation::Animation, error::structs::Error, platform::Platform};
use crate::commands::core::args::Arguments;
use crate::animations::animation::Process;


pub fn command() -> Command {
    command!("fetch").aliases(["ft"])
        .about("Fetch a repository from a configured platform")
        .args([
            arg!( -n --"dry-run" "Do everything except actually fetch the updates."),
            arg!([remote] "The name of the remote to fetch from")
                .add(Remote::complete())
            ,
            Arguments::pconf(false, true),
            Arguments::path_flag(false, "Path to the repository"),
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Initializing repository fetching...");
    
    let pconf = args.get_one::<Pconf>("pconf").map(| p| p.clone());
    
    let dry_run = args.get_flag("dry-run");
    
    let path = match args.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap()
    };
    
    let remote = match args.get_one::<String>("remote") {
        None => None,
        Some(e) => Some(e.to_owned())
    };
    
    let options = Options {
        method: Methods::DEFAULT ,
        remote, branch: None,
        force: false, dry_run
    };
    
    let result = Platform::fetch_repo(&path, pconf.clone(), options, &usettings, Some(&animation)).await;
    
    match result {
        Ok(logs) => {
            let message = match "" {
                _ if dry_run => cformat!("<y,i>dry-run</y,i> <g>finish without errors</>"),
                _ => cformat!("<y,i>fecth</y,i> <g>succeeded!</>")
            };
            
            animation.finish_with_success(message);
            for log in logs { println!("{log}"); }
        },
        Err(e) => {
            let action = Action::Fetch;
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
        }
    };
}
