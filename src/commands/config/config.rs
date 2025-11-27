use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;
use super::commands::{add,path,list};


pub fn command() -> Command {
    command!("config")
        .aliases(["cfg", "cf"])
        .about("Manage the configurations for grp")
        .subcommand(path::command())
        .subcommand(add::command())
        .subcommand(list::command())
}

pub fn manager(args: &ArgMatches) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("add", add) => add::manager(add),
            ("list", _) => list::manager(),
            ("path" , _) => path::manager(),
            _ => invalid()
        },
        _ => invalid()
    }
}

