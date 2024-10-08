// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::config::Config;
use crate::girep::platform::Platform;
use crate::update::metadata::Version;
use clap::crate_version;
use color_print::cformat;
use hyper::HeaderMap;

pub(crate) async fn get_latest_version() -> Result<Version, Error>{
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "reqwest > rust > grp".parse().unwrap());
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());

    let url = "https://api.github.com/repos/feraxhp/grp/releases/latest";

    let client = reqwest::Client::new();

    let result = match client.get(url).headers(headers).send().await {
        Ok(response) => response,
        Err(e) => {
            return Err(Error::new(
                ErrorType::Unknown,
                vec![
                    cformat!("<r>* Error while fetching the latest version</>").as_str(),
                    e.to_string().as_str(),
                ]
            ))
        }
    };

    let response_text = Platform::Github.error_manager(
        result,
        DebugData {
            rtype: Rtype::List,
            owner: "internal".to_string(),
            repo: None,
        },
        Config{
            pconf: "private".to_string(),
            user: "grp".to_string(),
            token: "⊘ ⊘ private ⊘ ⊘".to_string(),
            endpoint: "api.github.com".to_string(),
        },
        "".to_string(),
    ).await;

    let response_text = match response_text {
        Ok(text) => text,
        Err(e) => {
            return Err(e);
        }
    };

    let version: Version = match serde_json::from_str(&response_text.clone()) {
        Ok(org) => org,
        Err(e) => {
            return Err(
                Error::new(
                    ErrorType::Unknown,
                    vec![
                        format!("* Failed to parse the response: {}", e).as_str(),
                        format!("  Response: {}", response_text).as_str(),
                        cformat!("<y>* Unknown error</>").as_str(),
                    ]
                )
            )
        }
    };

    Ok(version)
}

pub(crate) async fn validate_version() -> Result<(bool, Version), Error> {
    let current_version = format!("v{}", crate_version!());
    let version = get_latest_version().await?;
    Ok((current_version >= version.name, version.clone()))
}