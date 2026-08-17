use clap::{command, ArgMatches, Command};

use crate::commands::core::common::invalid;
use crate::usettings::structs::Usettings;
use crate::commands::issues::commands::{list};


pub fn command() -> Command {
    command!("issues")
        .aliases(["is"])
        .about("Interface command to manage issues")
        .subcommand(list::command())
        // .subcommand(create::command())
        // .subcommand(delete::command())
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("list", args) => list::manager(args, usettings).await,
            // ("create", args) => create::manager(args, usettings).await,
            // ("delete", args) => delete::manager(args, usettings).await,
            _ => invalid()
        },
        _ => invalid()

    }
}