use crate::cmdcore::args::Arguments;
use crate::cmdcore::commands::Commands;
use clap::builder::ValueParser;
use clap::{arg, Command};

pub(crate) fn create_subcommand() -> Command {
    Commands::create("Create a new organization for the authenticated user")
        .args([
            Arguments::pconf(true, true),
            arg!(<name> "The name of the organization")
                .value_parser(ValueParser::string())
            ,
        ])
}