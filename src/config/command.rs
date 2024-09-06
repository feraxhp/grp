use crate::config::subcommands::add::{add_manager, add_subcommand};
use crate::config::subcommands::default::{default_manager, default_subcommand};
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
        .about("Manage the configuration file")
        .subcommand(add_subcommand())
        .subcommand(default_subcommand())
}
