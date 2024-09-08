// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::macros::validations::pconfs;

pub(crate) fn validate_repo_structure(value: &str) -> Result<String, String> {
    let (pconf, owner, repo_name) = match unfold_repo_structure(value) {
        Ok((pconf, owner, repo_name)) => (pconf, owner, repo_name),
        Err(e) => return Err(e)
    };

    let _ = match pconf {
        Some(string) => match pconfs::valid_pconfs(string.as_str()) {
            Ok(_) => { },
            Err(e) => return Err(e)
        },
        None => {  }
    };

    let _ = match owner {
        _ if owner.is_empty() => return Err(
            format!(
                "{} the owner cannot be empty\
                \n* if you want to use, the default owner, use the -\
                \n  e.g. <pconf>:*/<repo>",
                value
            )
        ),
        _ => { }
    };

    let _ = match repo_name {
        _ if repo_name.is_empty() => return Err(
            format!(
                "{} the repo name cannot be empty",
                value
            )
        ),
        _ => { }
    };

    Ok(value.to_string())
}

pub(crate) fn unfold_repo_structure(value: &str) -> Result<(Option<String>, String, String), String> {
    let (pconf, repo_full_name): (Option<String>, String) = match value.contains(":") {
        true => {
            let parts: Vec<&str> = value.split(':').collect();
            if parts.len() > 2 {
                return Err(
                    format!(
                        "{} is not a valid repo structure\
                        \n* the structure must follow [pconf]:<owner>/<repo>",
                        value
                    )
                );
            }
            let pconf = parts[0].to_string();
            let repo = parts[1].to_string();
            (Some(pconf), repo)
        },
        false => (None, value.to_string())
    };

    let (owner, repo_name): (String, String) = match repo_full_name.contains("/") {
        true => {
            let parts: Vec<&str> = repo_full_name.split('/').collect();
            if parts.len() != 2 {
                return Err(
                    format!(
                        "{} is not a valid repo structure\
                        \n* the structure must follow [pconf]:<owner>/<repo>",
                        value
                    )
                );
            }
            let owner = parts[0].to_string();
            let repo_name = parts[1].to_string();
            (owner, repo_name)
        },
        false => return Err(
            format!(
                "{} is not a valid repo structure\
                \n* the structure must follow [pconf]:<owner>/<repo>",
                value
            )
        )
    };

    Ok((pconf, owner, repo_name))
}