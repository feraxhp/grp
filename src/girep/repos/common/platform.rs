// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::repos::common::supported::Platform;
use color_print::cprintln;
use std::process::exit;

impl Platform {
    pub(crate) fn matches(name: &str) -> Platform {
        let platform: Platform = match name {
            "github" => Platform::Github,
            "gitea" => Platform::Gitea,
            name => {
                cprintln!("* Error: <i>{}</> is not a valid platform", name);
                exit(1)
            }
        };

        platform
    }
}