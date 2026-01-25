// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::{io, process::exit};

use color_print::cprintln;
use clap::{arg, command, ArgMatches, Command};
use grp_core::platform::SUPPORTED_REPOS;
use crate::usettings::structs::{Pconf, Usettings};


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
        .arg(arg!(-q --quiet "Add pconf without much dialog"))
        .arg(arg!([type] "The platform type (github, gitea, [more planed])")
            .value_parser(posible_values)
        )
        .arg(arg!([name] "The alias for the pconf"))
        .arg(arg!([owner] "The default owner to consult the platform"))
        .arg(arg!([token] "The token to access the platform"))
        .arg(arg!([endpoint] "The client endpoint to access the platform"))
}

pub(crate) fn manager(add: &ArgMatches) {

    let quiet = add.get_one::<bool>("quiet").unwrap_or(&false);
    let quiet = *quiet;
    let space = if quiet { "".to_string() } else { " ".repeat(4) };
    let format_exp = format!("{}Something went wrong while reading the input", space);

    let name = match add.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => {
            if !quiet {
                eprintln!("Name|");
                eprintln!("  Write the name of the repo");
                eprintln!("  This name will be used to identify the repo while using the CLI");
            }
            let mut name: Option<String> = None;
            while name.is_none() {
                eprint!("{}Name: ", space);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect(format_exp.as_str());

                let input = input.trim();

                if !input.is_empty() { name = Some(input.to_string()); }
                else { if !quiet { eprintln!("{space}Owner cannot be empty"); } }
            }

            name.unwrap()
        }
    };

    let owner = match add.get_one::<String>("owner") {
        Some(owner) => owner.to_string(),
        None => {
            if !quiet {
                eprintln!("Owner|");
                eprintln!("  Write the owner of the repo");
                eprintln!("  This owner will be used as the default owner of the repo");
            }

            let mut owner: Option<String> = None;
            while owner.is_none() {
                eprint!("{}Owner: ", space);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect(format_exp.as_str());

                let input = input.trim();

                if !input.is_empty() { owner = Some(input.to_string()); }
                else { if !quiet { eprintln!("{space}Owner cannot be empty"); } }
            }

            owner.unwrap()
        }
    };

    let token = match add.get_one::<String>("token") {
        Some(token) => token.to_string(),
        None => {
            if !quiet {
                eprintln!("Token|");
                eprintln!("  Write the token of the repo");
                eprintln!("  This token will be used to authenticate the CLI with the repo");
            }
            let mut token: Option<String> = None;
            while token.is_none() {
                eprint!("{}Token: ", space);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect(format_exp.as_str());

                let input = input.trim();

                if !input.is_empty() { token = Some(input.to_string()); }
                else { if !quiet { eprintln!("{space}Token cannot be empty"); } }
            }

            token.unwrap()
        }
    };

    let repo_type = match add.get_one::<String>("type") {
        Some(repo_type) => repo_type.to_string(),
        None => {
            if !quiet {
                eprintln!("type|");
                eprintln!("  Write the type of the repo");
                eprintln!("  options: ");
                for type_ in SUPPORTED_REPOS.iter() {
                    eprintln!("      {}) {}", type_.0, type_.2);
                }
            }

            let mut repo: Option<String> = None;
            while repo.is_none() {
                eprint!("{}Type: ", space);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect(format_exp.as_str());

                let input = input.trim();
                for type_ in SUPPORTED_REPOS.iter() {
                    if input == type_.0 || input == type_.1 || input == type_.2 {
                        repo = Some(type_.2.to_string());
                        break;
                    }
                }

                if repo.is_none() && !quiet {
                    eprintln!("{space}It is not a valid option");
                }
            }

            repo.unwrap()
        }
    };

    let endpoint = match add.get_one::<String>("endpoint") {
        Some(endpoint) => endpoint.to_string(),
        None => {
            let mut endpoint: Option<String> = match repo_type.as_str() {
                "github" => Some("api.github.com".to_string()),
                _ => None,
            };

            if endpoint.is_none() && !quiet {
                eprintln!("Endpoint|");
                eprintln!("  Write the endpoint of the repo");
                eprintln!("  This endpoint will be used to access the repo");
            }

            while endpoint.is_none() {
                eprint!("{}Endpoint: ", space);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect(format_exp.as_str());

                let input = input.trim();

                if !input.is_empty() { endpoint = Some(input.to_string()); }
                else { if !quiet { eprintln!("{space}Endpoint cannot be empty"); } }
            }

            endpoint.unwrap()
        }
    };

    let pconf = Pconf { name, owner, token, r#type: repo_type , endpoint };
    
    let mut usettings = match Usettings::read() {
        Ok(us) => us,
        Err(e) => {
            e.show();
            exit(1);
        },
    };
    
    match usettings.add_pconf(pconf) {
        Ok(_) => cprintln!("<g>* Succes: pconf saved succesfully</>"),
        Err(e) => {
            e.show();
            exit(1);
        },
    };
}

