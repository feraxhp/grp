// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::macros::validations::repo::validate_repo_structure;
use clap::builder::ValueParser;
use clap::{arg, command, Command};

pub(crate) fn create_subcommand() -> Command {
    let repo_validation = validate_repo_structure;
    command!("create")
        .aliases(["+", "cr"])
        .about("Create a new repository in a configured platform")
        .args([
            arg!(<repo> "The repository data as [pconf]:<owner>/<repo>")
                .value_parser(repo_validation)
                .required(true)
            ,
            arg!([description] "The description for the repository")
                .default_value("")
                .hide_default_value(true)
            ,
            arg!(-p --public "Make the repository public")
                .default_value_if("public", "false", "private")
                .default_value_if("public", "true", "public")
            ,
            arg!(-r --remote [path] "Add the remote to a local repository")
                .required(false)
                .require_equals(true)
                .value_hint(clap::ValueHint::FilePath)
                .value_parser(ValueParser::path_buf())
        ])
}