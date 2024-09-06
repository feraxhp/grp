// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;
mod macros;

use crate::config::command::config_manager;
use crate::girep::base::RepoProvider;
use clap::command;
use crate::macros::macros::invalid;

#[tokio::main]
async fn main() {
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .subcommand(config::command::config_command())
        .get_matches();

    let user_settings = config::loader::load_configurations();

    match commands.subcommand() {
        Some(sub) => match sub {
            ("config", config) => config_manager(config),
            _ => invalid()
        },
        _ => invalid()
    }
}
