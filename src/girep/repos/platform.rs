// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::structure::Pconf;
use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repos::gitea::implementation::Gitea;
use crate::girep::repos::github::implementation::Github;
use std::process::exit;

pub(crate) async fn get_platform(pconf: Pconf) -> Box<dyn Platform> {
    let config = Config::new(
        pconf.name,
        pconf.owner,
        pconf.token,
        pconf.endpoint
    );

    let platform: Box<dyn Platform> = match pconf.r#type.as_str() {
        "github" => Box::new(Github::new(config)),
        "gitea" => Box::new(Gitea::new(config)),
        _ => {
            eprintln!("Unknown provider");
            exit(101);
        }
    };

    platform
}