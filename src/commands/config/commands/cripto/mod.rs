mod encript;

use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;


pub fn command() -> Command {
    command!("cripto")
        .aliases(["ct", "cipher"])
        .about("Criper subcomands for grp")
        .subcommand(encript::command())
}

pub fn manager(args: &ArgMatches) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("encript", add) => encript::manager(add),
            _ => invalid()
        },
        _ => invalid()
    }
}
