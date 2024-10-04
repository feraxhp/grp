// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::macros::validations::pconfs::valid_pconfs_and_plus;
use clap::{arg, command, Command};

pub(crate) fn list_subcommand() -> Command {
    let possible_values = valid_pconfs_and_plus;

    command!("list")
        .alias("ls")
        .about("List the organizations for the authenticated user")
        .arg(
            arg!([pconf] "The pconf to list the organizations from")
                .value_parser(possible_values)
        )
}
