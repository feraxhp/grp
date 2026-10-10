mod encript;
mod decript;

use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;


pub fn command() -> Command {
    command!("cripto")
        .aliases(["ct", "cipher"])
        .about("Criper subcomands for grp")
        .subcommand(encript::command())
        .subcommand(decript::command())
}

pub fn manager(args: &ArgMatches) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("encript", args) => encript::manager(args),
            ("decript", args) => decript::manager(args),
            _ => invalid()
        },
        _ => invalid()
    }
}
