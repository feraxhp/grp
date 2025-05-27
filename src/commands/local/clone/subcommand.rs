// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{arg, command, Command};
use clap::builder::ValueParser;
use crate::commands::cmdcore::args::Arguments;

pub(crate) fn clone_subcommand() -> Command {
    command!("clone").aliases(["cl"])
        .about("Clone a repository from a configured platform")
        .args([
            Arguments::repo_structure(false, true),
            arg!([path] "The path to clone the repository")
                .value_hint(clap::ValueHint::DirPath)
                .value_parser(ValueParser::path_buf())
            ,
            arg!(-b --branch [name] "The name of the branch")
        ])
}