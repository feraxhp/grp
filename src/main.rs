// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;

use config::subcommands::add::set_configuration;
use crate::girep::base::RepoProvider;
use clap::{arg, command};

#[tokio::main]
async fn main() {
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage repositories")
        .subcommand(config::command::config_command())
        .get_matches();

    if let Some(config) = commands.subcommand_matches("config") {
        if let Some(add) = config.subcommand_matches("add") {
            set_configuration(add);
        }
    }
}
