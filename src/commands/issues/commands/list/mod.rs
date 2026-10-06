mod repo;
mod user;

use clap::Command;
use clap::ArgMatches;

use crate::commands::core::common::invalid;
use crate::usettings::structs::Usettings;
use crate::commands::core::commands::Commands;


pub fn command() -> Command {
    Commands::list("List issues for the given repo or authenticated user")
        .args(user::arguments())
        .subcommand(repo::command())
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("repo", args) => repo::manager(args, usettings).await,
            _ => invalid()
        },
        _ => user::manager(args, usettings).await
    }
}