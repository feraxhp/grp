// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::commands::cmdcore::args::Arguments;
use crate::commands::cmdcore::commands::Commands;
use clap::Command;

pub(crate) fn list_subcommand() -> Command {
    Commands::list("List the organizations for the authenticated user")
        .args([
            Arguments::pconf(false, true)
        ])
}
