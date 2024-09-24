use clap::{arg, command, Command};
use crate::macros::validations::repo::validate_repo_structure_with_pconf;

pub(crate) fn delete_subcommand() -> Command {
    let value_parser = validate_repo_structure_with_pconf;
    command!("delete")
        .aliases(&["del", "rm"])
        .about("Delete a repository")
        .arg(
            arg!(<repo> "The name of the repository to delete: <pconf>:<owner>/<repo>")
                .value_parser(value_parser)
        )
        .arg(
            arg!(-y --yes "Skip the confirmation prompt")
                .required(false)
        )
}