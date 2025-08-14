use std::path::PathBuf;
use clap::{arg, command, ArgMatches, Command, Arg};
use color_print::cformat;
use reqwest::Url;

use crate::girep::common::structs::Repo;
use crate::usettings::structs::Usettings;
use crate::local::git::structs::Action;
use crate::girep::{animation::Animation, common::show::Show, error::structs::Error, platform::Platform};
use crate::commands::core::{args::Arguments, utils::repo_struct::unfold_repo_structure};
use crate::usettings::validate::valid_pconfs;
use crate::animations::animation::Process;


pub fn command() -> Command {
    command!("clone").aliases(["cl"])
        .about("Clone a repository from a configured platform")
        .args([
            Arguments::repo_structure(false, true).conflicts_with("url"),
            Arg::new("url").short('u').long("url")
                .num_args(2)
                .value_names(["pconf", "url"])
                .conflicts_with("repo")
            ,
            Arguments::path(false, "The path to clone the repository"),
            arg!(-b --branch [name] "The name of the branch")
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Initializing repository cloning...");
    
    let path = args.get_one::<PathBuf>("path");
    
    let branch = match args.get_one::<String>("branch") {
        Some(value) => Some(value.clone()),
        None => None
    };
    
    match (args.get_one::<String>("repo"), args.get_many::<String>("url"))  {
        (Some(srepo), None) => by_repostructure(srepo, path, branch, &animation, usettings).await,
        (None, Some(values)) => {
            let mut values_iter = values.clone();
            let pconf = values_iter.next().unwrap().to_owned();
            let url = values_iter.next().unwrap().to_owned();
            
            match valid_pconfs(&pconf) {
                Ok(_) => {},
                Err(e) => {
                    animation.finish_with_error(e);
                    return ;
                },
            };
            
            let url = match Url::parse(url.as_str()) {
                Ok(u) => u,
                Err(e) => {
                    animation.finish_with_error(format!("{}", e));
                    return;
                },
            };
            
            by_url(url, path, pconf, branch, &animation, usettings).await;
        }
        _ => unreachable!()
    }
}

async fn by_repostructure<A: Animation + ?Sized>(srepo: &String, 
    path: Option<&PathBuf>, 
    branch: Option<String>, 
    animation: &Box<A>, 
    usettings: Usettings
) {
    let srepo = srepo.replace("\"", "");
    
    let (pconf, owner, repo) = 
        unfold_repo_structure(srepo.as_str(), false, &usettings).unwrap();
    
    let path = match path {
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap().join(repo.clone())
    };
    
    let pconf = match pconf {
        Some(e) => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        None => usettings.get_default_pconf().unwrap(),
    };
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    match platform.clone_repo(&owner, &repo, &path, branch, &config, Some(animation)).await {
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

async fn by_url<A: Animation + ?Sized>(url: Url, 
    path: Option<&PathBuf>, pconf: String,
    branch: Option<String>, 
    animation: &Box<A>, 
    usettings: Usettings
) {    
    let repo_ = match url.path_segments().iter().last() {
        Some(u) => u.clone().collect::<String>(),
        None => "defname".to_string(),
    };
    
    let path = match path {
        Some(value) => value.clone(),
        None => {
            std::env::current_dir().unwrap().join(repo_.clone())
        }
    };
    
    let pconf = usettings.get_pconf_by_name(pconf.as_str()).unwrap();
    
    let config = pconf.to_config();
    let url_string = url.to_string();
    
    match Platform::clone_by_url(&url_string, &path, branch, &config,  Some(animation)).await {
        Ok(_) => {
            animation.finish_with_success(cformat!("<y,i>clone</y,i> <g>succeeded!</>"));
            let repo = Repo {
                name: "".to_string(),
                path: path.as_os_str().to_str().unwrap().to_string(),
                private: None,
                url: url_string.clone(),
                git: url_string,
                description: None,
            };
            
            vec![repo].print_pretty();
        },
        Err(e) => {
            let action =  Action::Clone(url.host_str().unwrap().to_string());
            let repo = format!("{}", url);
            let error = Error::from_git2(e, action, &repo, Some(&config));
            
            animation.finish_with_error(&error.message);
            error.show();
        },
    }
}