// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::animations::process::Process;
use crate::config::structure::Usettings;
use crate::girep::repo::Repo;
use crate::girep::repos::local::remote::add_remote;
use crate::girep::repos::platform::get_platform;
use crate::macros::validations::repo_structure::unfold_repo_structure;
use crate::show;
use clap::ArgMatches;
use std::path::PathBuf;
use color_print::cprintln;

pub(crate) async fn create_mannager(ccreate: &ArgMatches, usettings: Usettings) {
    let srepo = ccreate.get_one::<String> ("repo").unwrap();
    let srepo = srepo.replace("\"", "");

    let description = ccreate.get_one::<String>("description").unwrap().to_string();

    let public = ccreate.get_one::<bool>("public").unwrap();

    let remote = ccreate.get_one::<PathBuf>("remote");

    let (pconf, owner, repo_name) = unfold_repo_structure(srepo.as_str(), false).unwrap();

    let pconf = match pconf {
        Some(value) => usettings.get_pconf(value).unwrap(),
        None => usettings.get_default()
    };

    let owner = match owner {
        value if value.eq("*") => pconf.owner.clone(),
        value => value
    };

    let platform = get_platform(pconf.clone()).await;

    let repo: Repo = platform.create_repo(owner, Repo {
        full_name: repo_name,
        description,
        state: if *public { "public".to_string() } else { "private".to_string() },
        html_url: "".to_string(),
        clone_url: "".to_string(),
    }).await;

    show!(vec![repo.clone()]);

    let _ = match remote {
        Some(value) => {
            let process_animation = Process::new("Adding remote to local repository");
            match add_remote(repo.clone_url.as_str(), pconf.r#type.as_str(),value.clone()) {
                Ok(_) => {
                    process_animation.finish_with_success("Remote added successfully");
                    cprintln!("  <m>* {}</>", pconf.r#type.clone());
                },
                Err(error) => {
                    process_animation.finish_with_error("Error adding remote");
                    cprintln!("\n  <r>* {}</>", error);
                }
            };
        },
        None => {}
    };
}