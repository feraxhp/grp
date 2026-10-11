// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cformat;
use clap::{arg, command, ArgMatches, Command};
use inquire::PasswordDisplayMode;

use grp_core::animation;
use grp_core::SUPPORTED_REPOS;

use crate::fail;
use crate::success;
use crate::system::input::Input;
use crate::system::cripto::Cripto;
use crate::usettings::structs::Pconf;
use crate::usettings::structs::Usettings;


pub(crate) fn command() -> Command {
    let names = SUPPORTED_REPOS.iter().map(|repo| repo.2);
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
        .arg(arg!(-e --encript "If pressent, it encripts the given token"))
        .arg(arg!(-d --"set-default" "If pressent, it sets the pconf as the default"))
        .arg(arg!([type])
            .help(cformat!("The platform type ({:?})", names))
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
        Err(e) => fail!(e),
    };
    
    let name = match add.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => {
            let description = cformat!("This name will be used to identify the repo while using the CLI");
            match Input::identifier(cformat!("<y>pconf <i>name</i>:</>"), Some(description)) {
                Ok(s) => s,
                Err(e) => fail!(e),
            }
        }
    };
    
    let owner = match add.get_one::<String>("owner") {
        Some(owner) => owner.to_string(),
        None => {
            let description = cformat!("This owner will be used as the default owner of the repo");
            match Input::identifier(cformat!("<y,i>owner</>"), Some(description)) {
                Ok(s) => s,
                Err(e) => fail!(e),
            }
        }
    };
    
    let mut token = match add.get_one::<String>("token") {
        Some(token) => token.to_string(),
        None => {
            let mode = match usettings.hidepass {
                true => PasswordDisplayMode::Hidden,
                false => PasswordDisplayMode::Masked,
            };
            let message = cformat!("<y,i>token</>");
            let confirmation: Option<String> = None;
            
            match Input::password( message, confirmation, mode) {
                Ok(s) => s,
                Err(e) => fail!(e),
            }
        }
    };
    
    let repo_type = match add.get_one::<String>("type") {
        Some(repo_type) => repo_type.to_string(),
        None => {
            let options = SUPPORTED_REPOS.iter().map(|s| s.2).collect();
            let description = cformat!("The API type to use when calling this pcong");
            
            match Input::select(cformat!("<y>repo <i>type</i>:</>"), Some(description), options) {
                Ok(s) => s,
                Err(e) => fail!(e),
            }
        }
    };
    
    let endpoint = match add.get_one::<String>("endpoint") {
        Some(endpoint) => endpoint.to_string(),
        None => {
            let description = cformat!("The actual enpoint to reach the API");
            
            match Input::endpoint(cformat!("<y, i>endpoint:</>"), Some(description), &repo_type) {
                Ok(s) => s,
                Err(e) => fail!(e),
            }
        }
    };

    let encript = add.get_flag("endpoint");

    if encript {
        let password = match usettings.get_password(false, &Box::new(animation::None)) {
            Ok(p) => p,
            Err(e) => fail!(e),
        };
        token = match Cripto::encript(&token, &password) {
            Ok(t) => t,
            Err(e) => fail!(e),
        }
    }
    let pconf = Pconf { name, owner, token, r#type: repo_type , endpoint, encripted: false };
    if add.get_flag("set-default") { usettings.default = pconf.name.clone() }
    
    match usettings.add_pconf(pconf) {
        Ok(_) => success!("add pconf"),
        Err(e) => fail!(e),
    };
}
