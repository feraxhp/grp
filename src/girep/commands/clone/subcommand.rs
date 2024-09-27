// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{arg, command, Command};
use clap::builder::ValueParser;
use crate::macros::validations::repo::validate_repo_structure;

pub(crate) fn clone_subcommand() -> Command {
    let repo_validation = validate_repo_structure;
    command!("clone")
        // .aliases(["c", "cl"])
        .about("Clone a repository from a configured platform")
        .args([
            arg!(<repo> "The repository data as [pconf]:<owner>/<repo>")
                .value_parser(repo_validation)
            ,
            arg!([path] "The path to clone the repository")
                .value_hint(clap::ValueHint::DirPath)
                .value_parser(ValueParser::path_buf())
            ,
            arg!(-b --branch [name] "The name of the branch")
        ])
}