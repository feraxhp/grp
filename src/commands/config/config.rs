use clap::{command, ArgMatches, Command};

use crate::commands::{config::commands::add, core::common::invalid};
use super::commands::path;

pub fn command() -> Command {
    command!("config")
        .aliases(["cfg", "cf"])
        .about("Manage the configurations for grp")
        .subcommand(path::command())
        .subcommand(add::command())
}

pub fn manager(args: &ArgMatches) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("add", add) => add::manager(add),
            // ("default", default) => default_manager(default),
            ("path" , _) => path::manager(),
            _ => invalid()
        },
        _ => invalid()
    }
}

