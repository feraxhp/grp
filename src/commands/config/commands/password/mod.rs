
use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;


pub fn command() -> Command {
    command!("password")
        .aliases(["pass", "passw", "pasw", "psw"])
        .about("Password releated configuration")
        // .subcommand(path::command())
        // .subcommand(add::command())
        // .subcommand(list::command())
        // .subcommand(cripto::command())
}

pub fn manager(args: &ArgMatches) {
    match args.subcommand() {
        Some(sub) => match sub {
            // ("add", add) => add::manager(add),
            // ("list", _) => list::manager(),
            // ("path" , _) => path::manager(),
            // ("cripto" , _) => cripto::manager(),
            _ => invalid()
        },
        _ => invalid()
    }
}