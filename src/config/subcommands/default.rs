// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::macros::validations::valid_pconfs;
use crate::config::loader::load_configurations;
use crate::config::structure::Pconf;
use clap::{arg, command, Command};
use std::process::exit;
use color_print::cprintln;

pub(crate) fn default_manager(default: &clap::ArgMatches) {
    let mut config = load_configurations();

    let list = default.get_one::<bool>("list").unwrap_or(&false);
    let list = *list;
    if list {
        list_pconfs(config.get_repos(), config.default.clone());
        exit(0);
    }

    let name = default.get_one::<String>("name").unwrap();
    config.set_default(name.clone());
}

pub(crate) fn default_subcommand() -> Command {
    let posible_values = valid_pconfs();

    command!("default")
        .about("Set the default pconf (platform configuration)")
        .arg(
            arg!(-l --list "List of the pconfs that can be set as default")
                .exclusive(true)
        )
        .arg(
            arg!(<name> "name of the pconf to set as default")
                .value_parser(posible_values)
        )
}

fn list_pconfs(repos: Vec<Pconf>, current: String) {
    let count = repos.len().to_string().len();

    let max_name = repos.iter().map(|repo| repo.name.len()).max().unwrap_or(0);
    let max_name = max_name.max("Name".len());

    let max_provider = repos.iter().map(|repo| repo.r#type.len()).max().unwrap_or(0);
    let max_provider = max_provider.max("Provider".len());

    let max_owner = repos.iter().map(|repo| repo.owner.len()).max().unwrap_or(0);
    let max_owner = max_owner.max("Owner".len());

    let width = 8 + max_owner + max_name + max_provider + count;

    eprintln!(
        "+{0:-<number$}+",
        "", number = count + width,
    );

    eprintln!(
        "{: <number$} | {: <max_name$} | {: <max_provider$} | {: <max_owner$}",
        "#" , "Name", "Provider", "Owner",
        number = count, max_name = max_name,
        max_provider = max_provider, max_owner = max_owner,
    );

    eprintln!(
        "+{0:-<number$}+",
        "", number = count + width,
    );

    for (index, repo) in repos.iter().enumerate() {
        if repo.name.clone() == current {
            cprintln!(
                "<bright-green>{: <number$} | {: <max_name$} | {: <max_provider$} | {: <max_owner$}</>",
                "*", repo.name, repo.r#type, repo.owner,
                number = count, max_name = max_name,
                max_provider = max_provider, max_owner = max_owner,
            )
        } else {
            eprintln!(
                "{: <number$} | {: <max_name$} | {: <max_provider$} | {: <max_owner$}",
                index, repo.name, repo.r#type, repo.owner,
                number = count, max_name = max_name,
                max_provider = max_provider, max_owner = max_owner,
            )
        };
    }

    // eprintln!(
    //     "{0:-<number$}",
    //     "", number = count + width,
    // );
}
