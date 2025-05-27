// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{command, Command};
use crate::commands::orgs::create::command::create_subcommand;
use crate::commands::orgs::delete::command::delete_subcommand;
use crate::commands::orgs::list::command::list_subcommand;

pub(crate) fn orgs_command() -> Command {
    command!("orgs")
        .aliases(["organizations", "org", "o"])
        .about("Interface command to manage organizations")
        .subcommands([
            list_subcommand(),
            create_subcommand(),
            delete_subcommand()
        ])
}
