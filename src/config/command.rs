use clap::{arg, command, Command};

pub(crate) fn config_command() -> Command {
    command!("config")
        .about("Manage the configuration file")
        .subcommand(
            command!("add")
                .about("Add a new repository to the configuration file interactively")
                .arg(arg!(-q --quiet "configure the repository without much interaction"))
                .arg(arg!([type] "The repository type (github, gitea, [more planed])")
                    .value_parser(["github", "gitea"])
                )
                .arg(arg!([name] "The alias for the repository configuration"))
                .arg(arg!([owner] "The default owner of the repository"))
                .arg(arg!([token] "The token to access the repository"))
                .arg(arg!([endpoint] "The client endpoint to access the repository"))
        )
}
