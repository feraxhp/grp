use std::path::PathBuf;
use clap::{ArgMatches, Command, arg, command};
use grp_core::animation::Animation;

use super::super::completions::structure::Completer;

use crate::commands::completions::git::remote::Remote;
use crate::local::structs::{Git2Context, Local, LocalError};
use crate::errors::ToError;
use crate::success;
use crate::usettings::structs::{Pconf, Usettings};
use crate::local::git::options::{Methods, Options};
use crate::local::git::structs::Action;
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
    let mut animation = Process::new("Initializing repository fetching...");
    
    let pconf = args.get_one::<Pconf>("pconf").cloned();
    
    let dry_run = args.get_flag("dry-run");
    
    let path = match args.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap()
    };
    
    let remote = args.get_one::<String>("remote").map(|e| e.to_owned());
    
    let options = Options {
        method: Methods::Default ,
        remote, branch: None,
        force: false, dry_run
    };
    
    let result = Local::fetch_repo(&path, pconf.clone(), options, &usettings, &mut animation).await;
    
    match result {
        Ok(logs) => {
            let message = match "" {
                _ if dry_run => success!(np "dry-run" "finish without errors"),
                _ => success!(np "fetch"),
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
                    let context = Git2Context {
                        action,
                        owner: &p.owner,
                        repo: path,
                        config: Some(&config),
                        usettings: &usettings,
                    };
                    
                    match e {
                        LocalError::Core(error) => error,
                        LocalError::Git(error) => error.to_error(context),
                    }
                }
                None => {
                    let pconf = usettings.get_default_pconf(&animation);
                    
                    let context = match pconf {
                        Ok(p) => {
                            Git2Context {
                                action,
                                owner: &p.owner.clone(),
                                repo: path,
                                config: None,
                                usettings: &usettings,
                            }
                        },
                        Err(e) => {
                            Git2Context {
                                action,
                                owner: &e.message().clone(),
                                repo: path,
                                config: None,
                                usettings: &usettings,
                            }
                        },
                    };
                    
                    match e {
                        LocalError::Core(error) => error,
                        LocalError::Git(error) => error.to_error(context),
                    }
                },
            };
            
            animation.finish_with_error(error.message());
            error.eprint();
        }
    };
}
