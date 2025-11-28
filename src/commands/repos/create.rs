use std::path::PathBuf;
use std::process::exit;

use clap::builder::ValueParser;
use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};
use crate::animations::animation::Create;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::commands::validations::repo::RepoStructure;
use crate::girep::animation::Animation;
use crate::girep::common::show::Show;
use crate::girep::common::structs::Repo;
use crate::girep::error::structs::Error;
use crate::girep::platform::Platform;
use crate::local::git::structs::Action;
use crate::system::directories::Directories;
use crate::girep::usettings::structs::Usettings;

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
    
    let path: Option<PathBuf> = match (add_to_local, remote) {
        (true, _) => match Directories::current_dir() {
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
    
    let platform = Platform::matches(pconf.r#type.as_str());
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
        Some(owner), repo, &config, Some(&animation)
    ).await;
    
    match result {
        Ok(repo) => {
            if let Some(path) = path {
                animation.change_message("Adding the remote");
                match platform.add_remote(&pconf.name, &repo.git, &path) {
                    Ok(_) => {
                        animation.finish_with_success(cformat!("<y,i>repo creation</y,i> <g>succeeded!</>"));
                        let name = match repo.private {
                            Some(true)  => cformat!("{} <r>priv</>", &repo.name),
                            Some(false) => cformat!("{} <g>pub </>", &repo.name),
                            None => unreachable!(),
                        };
                        cprintln!("<m>1.</> <g>Created repo:");
                        cprintln!("   <g>* <m>{} {}</>", name, repo.git);
                        cprintln!("<m>2.</> <g>Added remote: <y>{}</>", &pconf.name);
                    },
                    Err(e) => {
                        let action =  Action::SetRemote(pconf.name.clone(), repo.git.clone());
                        let path = path.as_os_str().to_str().unwrap_or("{{ Break path }}");
                        let error = Error::from_git2(e, action, &path, Some(&config));
                        animation.finish_with_warning(cformat!("Failed adding remote: <r>{}</>", &error.message));
                        let name = match repo.private {
                            Some(true)  => cformat!("{} <r>priv</>", &repo.name),
                            Some(false) => cformat!("{} <g>pub </>", &repo.name),
                            None => unreachable!(),
                        };
                        cprintln!("<m>1.</> <g>Created repo:");
                        cprintln!("   <g>* <m>{} {}</>", name, repo.git);
                        cprintln!("<m>2.</> <r>No added remote");
                        error.show_with_offset(3);
                    },
                }
            }
            else {
                animation.finish_with_success(cformat!("<y,i>repo creation</y,i> <g>succeeded!</>"));
                vec![repo].print_pretty();
            }
        },
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
        }
    }
}