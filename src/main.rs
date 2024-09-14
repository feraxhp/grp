// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;
mod macros;
mod animations;

use crate::config::command::{config_command, config_manager};
use crate::girep::commands::create::mannager::create_mannager;
use crate::girep::commands::create::subcommand::create_subcommand;
use crate::girep::commands::list::mannager::list_manager;
use crate::girep::commands::list::subcommand::list_subcommand;
use crate::macros::macros::invalid;
use clap::command;
use color_print::cprintln;
use crate::girep::commands::delete::mannager::delete_manager;
use crate::girep::commands::delete::subcommand::delete_subcommand;

#[tokio::main]
async fn main() {
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .subcommand(config_command())
        .subcommand(list_subcommand())
        .subcommand(create_subcommand())
        .subcommand(delete_subcommand())
        .get_matches();

    let user_settings = config::loader::load_configurations();

    match commands.subcommand() {
        Some(sub) => match sub {
            ("config", config) => config_manager(config),
            ("list", list) => list_manager(list, user_settings).await,
            ("create", create) => create_mannager(create, user_settings).await,
            ("delete", delete) => delete_manager(delete, user_settings).await,
            _ => invalid()
        },
        _ => {
            cprintln!("<y>* No command was provided try using <g,i>'--help'</>");
        }
    }
}
