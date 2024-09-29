// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::structure::Usettings;
use crate::girep::platform::Platform;
use crate::macros::validations::repo::unfold_repo_structure;
use clap::ArgMatches;
use color_print::cprintln;
use std::io;

pub(crate) async fn delete_manager(ccreate: &ArgMatches, usettings: Usettings) {
    let srepo = ccreate.get_one::<String>("repo").unwrap();
    let (pconf, owner, repo_name) = unfold_repo_structure(srepo.as_str(), true).unwrap();

    let mut confirmation = *ccreate.get_one::<bool>("yes").unwrap();

    let pconf = usettings.get_pconf(pconf.unwrap()).unwrap();

    let platform = Platform::matches(pconf.r#type.as_str());

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