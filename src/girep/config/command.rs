// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::config::subcommands::add::{add_manager, add_subcommand};
use crate::girep::config::subcommands::default::{default_manager, default_subcommand};
use clap::{command, Command};
use crate::invalid;

pub(crate) fn config_manager(config: &clap::ArgMatches) {
    match config.subcommand() {
        Some(sub) => match sub {
            ("add", add) => add_manager(add),
            ("default", default) => default_manager(default),
            _ => invalid()
        },
        _ => invalid()
    }
}

pub(crate) fn config_command() -> Command {
    command!("config")
        .aliases(["cfg"])
        .about("Manage the configurations for grp")
        .subcommand(add_subcommand())
        .subcommand(default_subcommand())
}
