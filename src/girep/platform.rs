// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cprintln;
use std::process::exit;
use hyper::HeaderMap;
use reqwest::Response;
use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::girep::common::repos::structs::DebugData;

pub(crate) const SUPPORTED_REPOS: [(&str, &str, &str); 2] = [
    ("0", "gh", "github"),
    ("1", "gt", "gitea"),
];

#[derive(PartialEq, Clone)]
pub(crate) enum Platform {
    Github,
    Gitea,
}

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

    pub fn get_auth_header(&self, token: String) -> HeaderMap {
        match self {
            Platform::Github => { crate::girep::github::header::get_auth_header(token) }
            Platform::Gitea => { crate::girep::gitea::header::get_auth_header(token) }
        }
    }

    pub fn get_base_url(&self, endpoint: String) -> String {
        match self {
            Platform::Github => { format!("https://{}", endpoint) }
            Platform::Gitea => { format!("https://{}/api/v1", endpoint) }
        }
    }

    pub async fn error_mannager(&self,
        result: Response,
        debug_data: DebugData,
        config: Config,
        base_message: String
    ) -> Result<String, Error> {
        match self {
            Platform::Github => { crate::girep::github::errors::error_manager(result, debug_data, config, base_message).await }
            Platform::Gitea => { crate::girep::gitea::errors::error_manager(result, debug_data, config, base_message).await }
        }
    }
}