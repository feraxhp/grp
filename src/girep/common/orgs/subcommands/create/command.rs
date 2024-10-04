use clap::{arg, command, Command};
use clap::builder::ValueParser;
use crate::macros::validations::pconfs::valid_pconfs_and_plus;

pub(crate) fn create_subcommand() -> Command {
    let possible_values = valid_pconfs_and_plus;

    command!("create")
        .aliases(["+", "cr"])
        .about("Create a new organization for the authenticated user")
        .args([
            arg!(<pconf> "The pconf to create the organization")
              .value_parser(possible_values)
            ,
            arg!(<name> "The name of the organization")
                .value_parser(ValueParser::string())
            ,
        ])
}