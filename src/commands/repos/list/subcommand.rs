// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::commands::cmdcore::args::Arguments;
use crate::commands::cmdcore::commands::Commands;
use clap::{arg, Command};

pub(crate) fn list_subcommand() -> Command {

    Commands::list("List the all the repositories from a configured repo")
        .args([
            Arguments::pconf(false, true),
            arg!([owner] "The platform to list the repositories from")
        ])
}