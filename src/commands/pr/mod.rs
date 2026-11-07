pub mod commands;

use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;
use crate::usettings::structs::Usettings;
use crate::commands::pr::commands::create;


pub fn command() -> Command {
    command!("pr")
        .aliases(["mr", "pull-request", "merge-recuest"])
        .about("Interface command to manage pull-request")
        .subcommand(create::command())
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("create", args) => create::manager(args, usettings).await,
            _ => invalid()
        },
        _ => invalid()
    }
}