// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cprintln;
use crate::config::save::save_config;
use crate::config::structure::Usettings;

pub(crate) fn load_configurations() -> Usettings {
    let file_location = crate::config::location::get_location();
    let file_location = file_location.as_str();

    let file = match std::fs::read_to_string(file_location) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("The config file could not be read");
            eprintln!("Error : {}", e);
            println!("Please check the config file at {}", file_location);
            std::process::exit(1);
        }
    };

    if file.is_empty() {
        let void_config = Usettings {
            default: "<repo-name>".to_string(),
            pconfs: vec![]
        };

        save_config(&void_config).unwrap();
        cprintln!(" The config file has been created at <i,u,b>{}</>", file_location);
        cprintln!(" * To configure run <bg:#333333, #ffffff>'grp config add'</>");

        std::process::exit(1);
    }

    let config: Usettings = match serde_json::from_str(&file) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("The config file is not valid");
            eprintln!("Error : {}", e);
            println!("Please check the config file at {}", file_location);
            std::process::exit(1);
        }
    };

    config
}
