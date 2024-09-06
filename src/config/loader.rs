// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::save::save_config;
use crate::config::structure::Root;

pub(crate) fn load_configurations() -> Root {
    let file_location = crate::config::location::get_location();
    let file_location = file_location.as_str();
    eprintln!("File location: {}", file_location);

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
        let void_config = Root {
            default: "<repo-name>".to_string(),
            repos: vec![]
        };

        save_config(&void_config).unwrap();
        println!("The config file has been created at {}", file_location);
        println!("To configure run grp config add");

        return void_config;
    }

    let config: Root = match serde_json::from_str(&file) {
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
