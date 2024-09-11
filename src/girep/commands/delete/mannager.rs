// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::io;
use std::process::exit;
use crate::config::structure::Usettings;
use crate::macros::validations::repo_structure::unfold_repo_structure;
use clap::ArgMatches;
use color_print::cprintln;
use crate::girep::repos::platform::get_platform;

pub(crate) async fn delete_manager(ccreate: &ArgMatches, usettings: Usettings) {
    let srepo = ccreate.get_one::<String>("repo").unwrap();
    let (pconf, owner, repo_name) = unfold_repo_structure(srepo.as_str(), true).unwrap();

    let mut confirmation = *ccreate.get_one::<bool>("yes").unwrap();

    let pconf = usettings.get_pconf(pconf.unwrap()).unwrap();

    let platform = get_platform(pconf.clone()).await;

    if !confirmation {
        eprintln!(
            "Do you realy whant to delete {}:{}/{}?",
            pconf.name, owner, repo_name
        );
    }
    while !confirmation {
        eprint!(
            "Type '{}/{}' to confirm: ",
            owner, repo_name
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong while reading the input");
        let input = input.trim();

        confirmation = input == format!("{}/{}", owner, repo_name);
    }

    if confirmation {
        match platform.delete_repo(owner, repo_name).await {
            true => cprintln!("Repository deleted successfully"),
            false => {
                exit(101)
            }
        };
    }
}