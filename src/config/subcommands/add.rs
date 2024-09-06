use crate::config::structure::Config;
use std::io;
use clap::{arg, command, ArgMatches, Command};

const SUPPORTED_REPOS: [(&str, &str, &str); 2] = [
    ("0", "gh", "github"),
    ("1", "gt", "gitea"),
];

pub(crate) fn add_manager(add: &ArgMatches) {

    let quiet = add.get_one::<bool>("quiet").unwrap_or(&false);
    let quiet = *quiet;

    let name = match add.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => {
            if !quiet {
                eprintln!("Name_______");
                eprintln!("Write the name of the repo");
                eprintln!("This name will be used to identify the repo while using the CLI");
            }
            let mut name: Option<String> = None;
            while name.is_none() {
                eprint!("Name: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Something went wrong while reading the input");

                let input = input.trim();

                if !input.is_empty() { name = Some(input.to_string()); }
                else { if !quiet { eprintln!("Name cannot be empty"); } }
            }

            name.unwrap()
        }
    };

    let owner = match add.get_one::<String>("owner") {
        Some(owner) => owner.to_string(),
        None => {
            if !quiet {
                eprintln!("Owner_______");
                eprintln!("Write the owner of the repo");
                eprintln!("This owner will be used as the default owner of the repo");
            }

            let mut owner: Option<String> = None;
            while owner.is_none() {
                eprint!("Owner: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Something went wrong while reading the input");

                let input = input.trim();

                if !input.is_empty() { owner = Some(input.to_string()); }
                else { if !quiet { eprintln!("Owner cannot be empty"); } }
            }

            owner.unwrap()
        }
    };

    let token = match add.get_one::<String>("token") {
        Some(token) => token.to_string(),
        None => {
            if !quiet {
                eprintln!("Token_______");
                eprintln!("Write the token of the repo");
                eprintln!("This token will be used to authenticate the CLI with the repo");
            }
            let mut token: Option<String> = None;
            while token.is_none() {
                eprint!("Token: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Something went wrong while reading the input");

                let input = input.trim();

                if !input.is_empty() { token = Some(input.to_string()); }
                else { if !quiet { eprintln!("Token cannot be empty"); } }
            }

            token.unwrap()
        }
    };

    let repo_type = match add.get_one::<String>("type") {
        Some(repo_type) => repo_type.to_string(),
        None => {
            if !quiet {
                eprintln!("type_______");
                eprintln!("Write the type of the repo");
                eprintln!("options: ");
                for type_ in SUPPORTED_REPOS.iter() {
                    eprintln!("{}: {}", type_.0, type_.2);
                }
            }

            let mut repo: Option<String> = None;
            while repo.is_none() {
                eprint!("Type: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Something went wrong while reading the input");

                let input = input.trim();
                for type_ in SUPPORTED_REPOS.iter() {
                    if input == type_.0 || input == type_.1 || input == type_.2 {
                        repo = Some(type_.2.to_string());
                        break;
                    }
                }

                if repo.is_none() && !quiet {
                    eprintln!("It is not a valid option");
                }
            }

            repo.unwrap()
        }
    };

    let endpoint = match add.get_one::<String>("endpoint") {
        Some(endpoint) => endpoint.to_string(),
        None => {
            let mut endpoint: Option<String> = match repo_type.as_str() {
                "github" => Some("api.github.com".to_string()),
                _ => None,
            };

            if endpoint.is_none() && !quiet {
                eprintln!("Endpoint_______");
                eprintln!("Write the endpoint of the repo");
                eprintln!("This endpoint will be used to access the repo");
            }

            while endpoint.is_none() {
                eprint!("Endpoint: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Something went wrong while reading the input");

                let input = input.trim();

                if !input.is_empty() { endpoint = Some(input.to_string()); }
                else { if !quiet { eprintln!("Endpoint cannot be empty"); } }
            }

            endpoint.unwrap()
        }
    };

    let config = Config::new(name, owner, token, repo_type, endpoint);
    config.save().expect("Impossible to save configuration");

}

pub(crate) fn add_subcommand() -> Command {
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
}