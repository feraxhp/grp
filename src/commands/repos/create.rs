use std::path::PathBuf;
use std::process::exit;
use grp_core::animation::Animation;
use grp_core::structs::Repo;
use grp_core::{Error, Formater, Platform};

use clap::builder::ValueParser;
use clap::{arg, ArgMatches, Command};
use color_print::{cformat};
use crate::animations::animation::Create;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::commands::validations::repo::RepoStructure;
use crate::local::structs::{Git2Error, Local};
use crate::system::show::Show;
use crate::local::git::structs::Action;
use crate::system::directories::BasicDir;
use crate::usettings::structs::Usettings;

pub fn command() -> Command {
    Commands::create("Create a new repository in a configured platform")
        .args([
            Arguments::repo_structure(false, true),
            arg!([description] "The description for the repository")
                .default_value("")
                .hide_default_value(true)
            ,
            arg!(-p --public "Make the repository public")
                .default_value_if("public", "false", "private")
                .default_value_if("public", "true", "public")
            ,
            arg!(-a --"add-to-local" "Add the remote to the current repository")
                .required(false)
                .conflicts_with("remote")
                .action(clap::ArgAction::SetTrue)
            ,
            arg!(-u --"set-upstream" "Add the remote as upstream for the current branch")
                .required(false)
                .requires("add-to-local")
                .conflicts_with("remote")
                // .action(clap::ArgAction::SetTrue)
            ,
            arg!(-r --remote [path] "Add the remote to a local repository")
                .required(false)
                .require_equals(true)
                .conflicts_with("add-to-local")
                .value_hint(clap::ValueHint::FilePath)
                .value_parser(ValueParser::path_buf())
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Create::new("Initializing repository creation...");
    
    let repo = args.get_one::<RepoStructure> ("repo").unwrap();
    let description = args.get_one::<String>("description").unwrap().to_string();
    let public = args.get_one::<bool>("public").unwrap();

    let remote = args.get_one::<PathBuf>("remote");
    let add_to_local = args.get_flag("add-to-local");
    let set_upstream = args.get_flag("set-upstream");
    
    let path: Option<PathBuf> = match (add_to_local, remote) {
        (true, _) => match BasicDir::current() {
            Ok(path) => Some(path),
            Err(e) => {
                animation.finish_with_error(&e.message);
                e.show();
                return;
            },
        },
        (_,Some(path)) => Some(path.clone()),
        (false, None) => None
    };
    
    let pconf = match repo.pconf.clone() {
        Some(e) => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        None => usettings.get_default_pconf().or_exit(&animation),
    };

    let owner = match repo.owner.clone() {
        value if value.eq("*") => pconf.owner.clone(),
        value => value
    };
    let platform = match Platform::matches(&pconf.r#type) {
        Ok(p) => p,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1)
        },
    };
    let platform = Local(platform);
    if let Err(e) = repo.is_unsupported(&platform) {
        animation.finish_with_error(&e.message);
        e.show();
        exit(1)
    }
    
    let config = pconf.to_config();
    
    let repo = Repo {
        name: repo.path.clone(),
        path: repo.path.clone(),
        private: Some(!public),
        url: String::new(),
        git: String::new(),
        description: Some(description),
    };
    
    let result = platform.create_repo(
        Some(&owner), repo, &config, &animation
    ).await;
    
    match result {
        Ok(repo) => {
            let name = match repo.private {
                Some(true)  => cformat!("{}   <r>priv</>", &repo.name),
                Some(false) => cformat!("{}   <g>pub </>", &repo.name),
                None => unreachable!(),
            };

            let mut finish_mode = FinishMode::SUCCESS;
            let mut messages = vec![
                cformat!("<m>1.</> <g>Created repo:"),
                format!(
                    "   {}", cformat!("<m>{}   {}</>", name, repo.git).as_tip()
                )
            ];

            let mut set_upstream_local = set_upstream;
            if let Some(path) = path {
                animation.change_message("Adding the remote");
                match platform.add_remote(&pconf.name, &repo.git, &path) {
                    Ok(_) => {
                        set_upstream_local = true && set_upstream_local;
                        messages.push(
                            cformat!("<m>2.</> <g>Added remote: <y>{}</>", &pconf.name)
                        );
                    },
                    Err(e) => {
                        set_upstream_local = false;
                        let action =  Action::SetRemote(pconf.name.clone(), repo.git.clone());
                        let path = path.as_os_str().to_str().unwrap_or("{{ Break path }}");
                        let error = Error::from_git2(e, action, &owner, &path, Some(&config), &usettings);
                        finish_mode = FinishMode::WARNING(cformat!("Failed adding remote: <r>{}</>", &error.message));
                        
                        messages.push(cformat!("<m>2.</> <r>No added remote"));
                        messages.extend(
                            error.to_string_iter(&3)
                        );
                    },
                }
                
                match (set_upstream_local, set_upstream) {
                    (true, true) => {
                        match platform.set_upstream_to_local_branch(&pconf.name, &path) {
                            Ok(branch) => {
                                messages.push(cformat!("<m>3.</> <g>upstream setted"));
                                messages.push(
                                    format!(
                                        "   {}", cformat!("<m>{} {}</>", &pconf.name, branch).as_tip()
                                    )
                                );
                            },
                            Err(e) => {
                                let action =  Action::SetUpstream;
                                let path = path.as_os_str().to_str().unwrap_or("{{ Break path }}");
                                let error = Error::from_git2(e, action, &owner, &path, Some(&config), &usettings);
                                finish_mode = FinishMode::WARNING(cformat!("Failed setting upstream: <r>{}</>", &error.message));
                                
                                messages.push(cformat!("<m>3.</> <r>No upstream set"));
                                messages.extend(
                                    error.to_string_iter(&3)
                                );
                            },
                        };
                    },
                    (false, true) => {
                        messages.push(cformat!("<m>3.</> <y>Skiping set upstream due to previous error"));
                    }
                    _ => ()
                }
            }
            else {
                messages.extend(
                    vec![repo].to_string_iter()
                );
            }
            
            match finish_mode {
                FinishMode::SUCCESS => animation.finish_with_success(cformat!("<y,i>repo creation</y,i> <g>succeeded!</>")),
                FinishMode::WARNING(msg) => animation.finish_with_warning(msg),
            };
            
            messages.iter().for_each(|e| {
                println!("{}", e)
            });
        },
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
        }
    }
}

enum FinishMode {
    SUCCESS,
    WARNING(String),
}

#[test]
fn no_match_upstream() {
    let res = command().try_get_matches_from([
        "create", "-u", "gh:create/something"
    ]);
    
    assert!(res.is_err())
}

#[test]
fn match_upstream() {
    let res = command().try_get_matches_from([
        "create", "-au", "gh:create/something"
    ]);
    
    assert!(res.is_ok())
}
