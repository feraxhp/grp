use crate::cmdcore::args::Arguments;
use crate::cmdcore::commands::Commands;
use clap::{arg, Command};

pub(crate) fn delete_subcommand() -> Command {
    Commands::delete("Delete a repository")
        .args([
            Arguments::repo_structure(true, true),
            arg!(-y --yes "Skip the confirmation prompt")
                .required(false)
        ])
}
