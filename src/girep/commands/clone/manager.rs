// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::path::PathBuf;
use clap::ArgMatches;
use color_print::cprintln;
use crate::config::structure::Usettings;
use crate::girep::repos::common::supported::Platform;
use crate::macros::validations::repo::unfold_repo_structure;
use crate::show;

pub(crate) fn clone_manager(ccreate: &ArgMatches, usettings: Usettings) {
    let srepo = ccreate.get_one::<String> ("repo").unwrap();
    let srepo = srepo.replace("\"", "");

    let (pconf, owner, repo) = unfold_repo_structure(srepo.as_str(), false).unwrap();

    let path = match ccreate.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap().join(repo.clone())
    };

    let branch = match ccreate.get_one::<String>("branch") {
        Some(value) => Some(value.clone()),
        None => None
    };


    let pconf = match pconf {
        Some(value) => usettings.get_pconf(value).unwrap(),
        None => usettings.get_default()
    };

    let platform = Platform::matches(pconf.r#type.as_str());

    // cprintln!(
    //     "Cloning repository {} branch {} from {} to {}",
    //     repo,
    //     branch.unwrap_or(&"default".to_string()),
    //     owner,
    //     path.display()
    // );

    let repo = platform.clone_repo(owner, repo, path, branch, pconf.to_conf());

    match repo {
        Ok(r) => {
            show!(vec![r.clone()]);
        },
        Err(e) => {
            e.show();
            std::process::exit(1);
        }
    }
}