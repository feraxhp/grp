// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::io;
use std::process::exit;
use crate::config::structure::Usettings;
use crate::macros::validations::repo_command::unfold_repo_structure;
use clap::ArgMatches;
use color_print::cprintln;
use crate::girep::repos::common::supported::Platform;
use crate::girep::repos::platform::get_platform;

pub(crate) async fn delete_manager(ccreate: &ArgMatches, usettings: Usettings) {
    let srepo = ccreate.get_one::<String>("repo").unwrap();
    let (pconf, owner, repo_name) = unfold_repo_structure(srepo.as_str(), true).unwrap();

    let mut confirmation = *ccreate.get_one::<bool>("yes").unwrap();

    let pconf = usettings.get_pconf(pconf.unwrap()).unwrap();

    let platform = match pconf.r#type.as_str() {
        "github" => Platform::Github,
        "gitea" => Platform::Gitea,
        _ => {
            cprintln!("* Error: <i>{}</> is not a valid platform", pconf.r#type.clone());
            exit(1)
        }
    };

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
        match platform.delete_repo(owner, repo_name, pconf.to_conf()).await {
            Ok(_) => cprintln!("Repository deleted successfully"),
            Err(e) => {
                e.show();
            }
        };
    }
}