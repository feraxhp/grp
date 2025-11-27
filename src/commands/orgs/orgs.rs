use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;
use crate::girep::usettings::structs::Usettings;
use crate::commands::orgs::commands::{create, delete, list};


pub fn command() -> Command {
    command!("orgs")
        .aliases(["organizations", "org", "o"])
        .about("Interface command to manage organizations")
        .subcommand(list::command())
        .subcommand(create::command())
        .subcommand(delete::command())
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("list", args) => list::manager(args, usettings).await,
            ("create", args) => create::manager(args, usettings).await,
            ("delete", args) => delete::manager(args, usettings).await,
            _ => invalid()
        },
        _ => invalid()

    }
}