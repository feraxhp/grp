// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::path::PathBuf;

pub fn get_location() -> String {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let location = match std::env::consts::OS {
        "linux" => home_dir.join(".config/girep/config.json"),
        "windows" => PathBuf::from(std::env::var("APPDATA").unwrap()).join("girep/config.json"),
        "macos" => home_dir.join("Library/Application Support/girep/config.json"),
        _ => home_dir.join(".config/girep/config.json"),
    };

    // Create file if it does not exist
    if !location.exists() {
        if let Some(parent) = location.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create config directory");
        }
        std::fs::File::create(&location).expect("Failed to create config file");
    }

    location.to_str().unwrap().to_string()
}