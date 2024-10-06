// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{arg, Command};
use clap::builder::ValueParser;
use crate::cmdcore::args::Arguments;
use crate::cmdcore::commands::Commands;

pub(crate) fn delete_subcommand() -> Command {
    Commands::delete("Delete a organization")
        .args([
            Arguments::pconf(true, false),
            arg!(<name> "The name of the organization")
                .value_parser(ValueParser::string())
            ,
            arg!(-y --yes "Skip the confirmation prompt")
                .required(false)
            ,
        ])
}
