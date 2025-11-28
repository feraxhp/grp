use std::path::PathBuf;
use std::process::exit;
use clap::{Arg, ArgMatches, Command, arg, command};
use color_print::cformat;
use reqwest::Url;

use super::super::completions::structure::Completer;

use crate::commands::validations::repo::RepoStructure;
use crate::girep::common::structs::Repo;
use crate::local::clone::CloneOptions;
use crate::girep::usettings::structs::Usettings;
use crate::local::git::structs::Action;
use crate::girep::{animation::Animation, common::show::Show, error::structs::Error, platform::Platform};
use crate::commands::core::args::Arguments;
use crate::girep::usettings::validate::valid_pconfs;
use crate::animations::animation::Process;

const DEFNAME: &'static str = "defname";

pub fn command() -> Command {
    command!("clone").aliases(["cl"])
        .about("Clone a repository from a configured platform")
        .args([
            Arguments::repo_structure(false, true).conflicts_with("url"),
            Arg::new("url").short('u').long("url")
                .num_args(2)
                .value_names(["pconf", "url"])
                .add(Usettings::complete())
            ,
            Arguments::path(false, "The path to clone the repository"),
            arg!(-b --branch [name] "The name of the branch"),
            arg!(-B --bare "Clone as bare repo")
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Initializing repository cloning...");
    
    let path = args.get_one::<PathBuf>("path");
    
    let branch = match args.get_one::<String>("branch") {
        Some(value) => Some(value.clone()),
        None => None
    };
    
    let bare = args.get_flag("bare");
    
    match (args.get_one::<RepoStructure>("repo"), args.get_many::<String>("url"))  {
        (Some(repo), None) => by_repostructure(repo, bare, path, branch, &animation, usettings).await,
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
            
            by_url(url, bare, path, pconf, branch, &animation, usettings).await;
        }
        _ => unreachable!()
    }
}

async fn by_repostructure<A: Animation + ?Sized>(repo: &RepoStructure, 
    bare: bool,
    path: Option<&PathBuf>, 
    branch: Option<String>, 
    animation: &Box<A>, 
    usettings: Usettings
) {
    let pconf = match &repo.pconf {
        Some(e) => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        None => usettings.default_or_exit(&animation),
    };
    
    let path = match path {
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap().join(&repo.path)
    };
    
    let platform = Platform::matches(pconf.r#type.as_str());
    if let Err(e) = repo.is_unsupported(&platform) {
        animation.finish_with_error(&e.message);
        e.show();
        exit(1)
    }
    let config = pconf.to_config();
    
    let options = CloneOptions {
        path: path,
        branch: branch,
        bare: bare,
    };
    
    match platform.clone_repo(&repo.owner, &repo.path, &options, &config, Some(animation)).await {
        Ok(r) => {
            animation.finish_with_success(cformat!("<y,i>clone</y,i> <g>succeeded!</>"));
            vec![r].print_pretty();
        },
        Err(e) => {
            let action =  Action::Clone(platform.name().to_string());
            let repo = format!("{}/{}", &repo.owner, &repo.path);
            let error = Error::from_git2(e, action, &repo, Some(&config));
            
            animation.finish_with_error(&error.message);
            error.show();
        },
    }
}

async fn by_url<A: Animation + ?Sized>(url: Url, 
    bare: bool,
    path: Option<&PathBuf>, pconf: String,
    branch: Option<String>, 
    animation: &Box<A>, 
    usettings: Usettings
) {
    let repo_ = match url.path_segments() {
        Some(segments) => {
            let segments_vec: Vec<&str> = segments.collect();
            segments_vec.last().unwrap_or(&DEFNAME).to_string()
        }
        None => DEFNAME.to_string(),
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
    
    let options = CloneOptions {
        path: path.clone(),
        branch: branch,
        bare: bare,
    };
    
    match Platform::clone_by_url(&url_string, &options, &config,  Some(animation)).await {
        Ok(_) => {
            animation.finish_with_success(cformat!("<y,i>clone</y,i> <g>succeeded!</>"));
            let repo = Repo {
                name: "".to_string(),
                path: repo_,
                private: None,
                url: path.as_os_str().to_str().unwrap().to_string(),
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