// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;
mod macros;
mod animations;

use crate::config::command::{config_command, config_manager};
use crate::girep::commands::list::{list_manager, list_subcommand};
use crate::girep::base::Platform;
use clap::command;
use crate::macros::macros::invalid;

#[tokio::main]
async fn main() {
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .subcommand(config_command())
        .subcommand(list_subcommand())
        .get_matches();

    let user_settings = config::loader::load_configurations();

    match commands.subcommand() {
        Some(sub) => match sub {
            ("config", config) => config_manager(config),
            ("list", list) => list_manager(list, user_settings).await,
            _ => invalid()
        },
        _ => invalid()
    }
}
