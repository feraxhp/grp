// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{command, Command};
use crate::girep::common::orgs::subcommands::create::command::create_subcommand;
use crate::girep::common::orgs::subcommands::list::command::list_subcommand;

pub(crate) fn orgs_command() -> Command {
    command!("orgs")
        .aliases(["organizations", "org", "o"])
        .about("Interface command to manage organizations")
        .subcommand(list_subcommand())
        .subcommand(create_subcommand())
}
