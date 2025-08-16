use std::{io::{self, Write}, process::exit};

use clap::crate_version;


pub fn show_version(value: &str) -> Result<bool, String> {
    match value {
        "true" => {
            let version = crate_version!();
            let _ = io::stdout().write(version.as_bytes());
            let _ = io::stdout().flush();
            
            println!();
            exit(0)
        },
        "false" => Ok(false),
        _ => unreachable!()
    }
}