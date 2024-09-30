// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;
mod macros;
mod animations;
pub(crate) mod errors;

use crate::config::command::{config_command, config_manager};
use girep::common::repos::commands::create::mannager::create_mannager;
use girep::common::repos::commands::create::subcommand::create_subcommand;
use girep::common::repos::commands::delete::mannager::delete_manager;
use girep::common::repos::commands::delete::subcommand::delete_subcommand;
use girep::common::repos::commands::list::mannager::list_manager;
use girep::common::repos::commands::list::subcommand::list_subcommand;
use crate::macros::macros::invalid;
use clap::{arg, command, crate_version};
use color_print::cprintln;
use std::io;
use std::io::Write;
use std::process::exit;
use girep::common::repos::commands::clone::manager::clone_manager;
use girep::common::repos::commands::clone::subcommand::clone_subcommand;

#[tokio::main]
async fn main() {
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .arg(arg!(-v --vnumber "Prints the version number to the standar output").exclusive(true))
        .subcommand(config_command())
        .subcommand(list_subcommand())
        .subcommand(create_subcommand())
        .subcommand(delete_subcommand())
        .subcommand(clone_subcommand())
        .get_matches();


    match commands.clone().args_present() {
        true => {
            if *commands.get_one::<bool>("vnumber").unwrap_or(&false) {
                let version = crate_version!();
                let _ = io::stdout().write(version.as_bytes());
                println!();
                exit(0);
            }
        },
        _ => {}
    }

    let user_settings = config::loader::load_configurations();

    match commands.subcommand() {
        Some(sub) => match sub {
            ("config", config) => config_manager(config),
            ("list", list) => list_manager(list, user_settings).await,
            ("create", create) => create_mannager(create, user_settings).await,
            ("delete", delete) => delete_manager(delete, user_settings).await,
            ("clone", clone) => clone_manager(clone, user_settings).await,
            _ => invalid()
        },
        _ => {
            cprintln!("<y>* No command was provided try using <g,i>'--help'</>");
        }
    }
}
