// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::{io, process::exit};

use clap::{arg, command, ArgMatches, Command};
use color_print::{cformat, cprintln};
use grp_core::SUPPORTED_REPOS;
use inquire::PasswordDisplayMode;

use crate::{system::input::Input, usettings::structs::{Pconf, Usettings}};


pub(crate) fn command() -> Command {
    let posible_values = |value: &str| {
        let names = SUPPORTED_REPOS.iter().map(|repo| repo.2.to_string()).collect::<Vec<String>>();
        let abbreviations = SUPPORTED_REPOS.iter().map(|repo| repo.1.to_string()).collect::<Vec<String>>();


        match &value.to_string(){
            value if names.contains(&value) => Ok(value.to_string()),
            value if abbreviations.contains(&value) => {
                let index = abbreviations.iter().position(|r| r.eq(value)).unwrap();
                Ok(names[index].to_string())
            },
            _ => Err(
                format!(
                    "{} is not a valid pconf type\n\
                    posible values are {:?}",
                    value, names
                )
            )
        }
    };
    
    command!("add")
        .aliases(["+"])
        .about("Add a new pconf interactively")
        .arg(arg!([type] "The platform type (github, gitea, [more planed])")
            .value_parser(posible_values)
        )
        .arg(arg!([name] "The alias for the pconf"))
        .arg(arg!([owner] "The default owner to consult the platform"))
        .arg(arg!([token] "The token to access the platform"))
        .arg(arg!([endpoint] "The client endpoint to access the platform"))
}

pub(crate) fn manager(add: &ArgMatches) {
    let mut usettings = match Usettings::read() {
        Ok(us) => us,
        Err(e) => {
            e.show();
            exit(1);
        },
    };
    
    let name = match add.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => {
            let description = cformat!("This name will be used to identify the repo while using the CLI");
            match Input::identifier(cformat!("<y>pconf <i>name</i>:</>"), Some(description)) {
                Ok(s) => s,
                Err(e) => {
                    cprintln!("<r>{}</>", &e.message);
                    e.show();
                    exit(1)
                },
            }
        }
    };
    
    let owner = match add.get_one::<String>("owner") {
        Some(owner) => owner.to_string(),
        None => {
            let description = cformat!("This owner will be used as the default owner of the repo");
            match Input::identifier(cformat!("<y,i>owner</>"), Some(description)) {
                Ok(s) => s,
                Err(e) => {
                    cprintln!("<r>{}</>", &e.message);
                    e.show();
                    exit(1)
                },
            }
        }
    };
    
    let token = match add.get_one::<String>("token") {
        Some(token) => token.to_string(),
        None => {
            let mode = match usettings.hidepass {
                true => PasswordDisplayMode::Hidden,
                false => PasswordDisplayMode::Masked,
            };
            let message = cformat!("<y,i>password</>");
            let confirmation = Some(cformat!("<y,i>password</>"));
            
            match Input::password( message, confirmation, mode) {
                Ok(s) => s,
                Err(e) => {
                    cprintln!("<r>{}</>", &e.message);
                    e.show();
                    exit(1)
                },
            }
        }
    };
    
    let repo_type = match add.get_one::<String>("type") {
        Some(repo_type) => repo_type.to_string(),
        None => {
            let description = cformat!("This owner will be used as the default owner of the repo");
            let options = SUPPORTED_REPOS.iter().map(|s| s.2).collect();
            
            match Input::select(cformat!("<y>repo <i>type</i>:</>"), Some(description), options) {
                Ok(s) => s,
                Err(e) => {
                    cprintln!("<r>{}</>", &e.message);
                    e.show();
                    exit(1)
                },
            }
        }
    };
    
    let endpoint = match add.get_one::<String>("endpoint") {
        Some(endpoint) => endpoint.to_string(),
        None => {
            let mut placeholder: Option<String> = match repo_type.as_str() {
                "github" => Some("api.github.com".to_string()),
                _ => None,
            };
            
            
        }
    };

    let pconf = Pconf { name, owner, token, r#type: repo_type , endpoint, encripted: false };
    
    match usettings.add_pconf(pconf) {
        Ok(_) => cprintln!("<g>* Succes: pconf saved succesfully</>"),
        Err(e) => {
            e.show();
            exit(1);
        },
    };
}

