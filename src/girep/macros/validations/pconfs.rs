// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::config::loader::load_configurations;

fn internal(value: &str, vec: Vec<&str>) -> Result<String, String> {
    let config = load_configurations();
    let repos = config.get_repos();
    let mut names: Vec<String> = repos.iter().map(|repo| repo.name.clone()).collect();
    let mut add = vec.iter().map(|repo| repo.to_string()).collect();
    names.append(&mut add);
    if names.contains(&value.to_string()) {
        Ok(value.to_string())
    } else {
        Err(
            format!(
                "{} is not a valid pconf name\n\
                    possible values are {:?}",
                value, names
            )
        )
    }
}

pub(crate) fn valid_pconfs(value: &str) -> Result<String, String> {
    internal(value, vec![])
}

pub(crate) fn valid_pconfs_and_plus(value: &str) -> Result<String, String> {
    internal(value, vec!["-"])
}