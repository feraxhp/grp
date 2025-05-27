// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::io;
use std::process::exit;
use clap::ArgMatches;
use color_print::cprintln;
use crate::girep::config::structure::Usettings;
use crate::girep::platform::Platform;
use crate::girep::macros::validations::repo::unfold_repo_structure;

pub(crate) async fn delete_manager(ccreate: &ArgMatches, usettings: Usettings) {

    let pconf = usettings.matches_pconf(ccreate.get_one::<String>("pconf"));

    let mut confirmation = *ccreate.get_one::<bool>("yes").unwrap();

    let name = match ccreate.get_one::<String>("name") {
        Some(name) => name,
        None => {
            eprintln!("* You must provide a name for the organization");
            exit(1)
        }
    };

    let platform = Platform::matches(pconf.r#type.as_str());

    if !confirmation {
        cprintln!(
            "Do you really want to delete the <r,i>{}</>?",
            name.clone()
        );
    }

    while !confirmation {
        eprint!("Type '{}': ", name );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong while reading the input");
        let input = input.trim();

        confirmation = input == name.clone()
    }

    if confirmation {
        match platform.delete_org(name.to_string(), pconf.to_conf()).await {
            Ok(_) => (),
            Err(e) => {
                e.show();
            }
        };
    }
}
