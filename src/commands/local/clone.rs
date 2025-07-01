use std::path::PathBuf;
use clap::{arg, command, ArgMatches, Command};
use color_print::cformat;

use crate::usettings::structs::Usettings;
use crate::local::git::structs::Action;
use crate::girep::{animation::Animation, common::show::Show, error::structs::Error, platform::Platform};
use crate::commands::core::{args::Arguments, utils::repo_struct::unfold_repo_structure};
use crate::animations::animation::Process;


pub fn command() -> Command {
    command!("clone").aliases(["cl"])
        .about("Clone a repository from a configured platform")
        .args([
            Arguments::repo_structure(false, true),
            Arguments::path(false, "The path to clone the repository"),
            arg!(-b --branch [name] "The name of the branch")
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Initializing repository cloning...");
    let srepo = args.get_one::<String> ("repo").unwrap();
    let srepo = srepo.replace("\"", "");
    
    let (pconf, owner, repo) = 
        unfold_repo_structure(srepo.as_str(), false, &usettings).unwrap();
    
    let path = match args.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap().join(repo.clone())
    };
    
    let branch = match args.get_one::<String>("branch") {
        Some(value) => Some(value.clone()),
        None => None
    };
    
    let pconf = match pconf {
        Some(e) => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        None => usettings.get_default_pconf().unwrap(),
    };
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    match platform.clone(&owner, &repo, &path, branch, &config, Some(&animation)).await {
        Ok(r) => {
            animation.finish_with_success(cformat!("<y,i>clone</y,i> <g>succeeded!</>"));
            vec![r].print_pretty();
        },
        Err(e) => {
            let action =  Action::Clone(platform.name().to_string());
            let repo = format!("{}/{}", owner, repo);
            let error = Error::from_git2(e, action, &repo, Some(&config));
            
            animation.finish_with_error(&error.message);
            error.show();
        },
    }
}