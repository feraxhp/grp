// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{arg, command, ArgMatches, Command};
use crate::config::structure::Usettings;
use crate::girep::repos::platform::get_platform;
use crate::macros::validations::valid_pconfs;
use crate::show;

pub(crate) async fn list_manager(clist: &ArgMatches, usettings: Usettings) {

    let pconf = match clist.get_one::<String>("pconf"){
        Some(clist) => {
            match usettings.get_pconf(clist.clone()) {
                Some(pconf) => pconf,
                None => {
                    let pconf = usettings.get_default();
                    if !clist.eq("-") {
                        eprintln!("{} is not a valid pconf name", clist);
                        eprintln!("using default pconf: {}", pconf.name.clone());
                    }
                    pconf.clone()
                },
            }
        },
        None => { usettings.get_default() }
    };

    let platform = get_platform(pconf.clone()).await;

    let repos = match clist.get_one::<String>("owner") {
        Some(owner) => platform.list_repos(Some(owner.clone())).await,
        None => platform.list_repos(None).await
    };

    show!(repos);
}

pub(crate) fn list_subcommand() -> Command {
    let posible_values = valid_pconfs();

    command!("list")
        .alias("ls")
        .about("List the all the repositories from a configured repo")
        .arg(
            arg!([pconf] "The pconf to list the repositories from")
                .value_parser(posible_values)
                .value_parser(["-"])
        )
        .arg(
            arg!([owner] "The platform to list the repositories from")
        )
}