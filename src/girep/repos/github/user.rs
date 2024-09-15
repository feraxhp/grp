// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repos::github::implementation::Github;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use std::os::linux::raw::stat;
use crate::girep::repos::user_type::UserType;

#[derive(Deserialize)]
struct Transpiler {
    login: String,
}

pub(crate) async fn get_user_type(name: &str, conf: Config) -> Result<UserType, Error> {
    match is_logged_user(name, conf.clone()).await {
        Ok(true) => Ok(UserType::Logged),
        Ok(false) => {
            match is_organization(name, conf).await {
                Ok(true) => Ok(UserType::Organization),
                Ok(false) => Ok(UserType::Free),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}


pub(crate) async fn is_logged_user(name: &str, conf: Config) -> Result<bool, Error> {
    let client = reqwest::Client::new();

    let result = client
        .get(format!("https://{}/user", conf.endpoint))
        .headers(Github::get_auth_header(conf.token.clone()))
        .send()
        .await
        .map_err(
            |e| {
                Error::new(ErrorKind::Other, e.to_string())
            }
        )?;

    match result.status().as_u16() {
        200 => {
            let transpiler: Transpiler = result
                .json().await
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            Ok(transpiler.login == name)
        },
        401 => {
            Err(Error::new(ErrorKind::Other, "Unauthorized"))
        },
        _ => {
            Err(Error::new(ErrorKind::Other, "Failed while fetching user type"))
        }
    }

}

async fn is_organization(name: &str, conf: Config) -> Result<bool, Error> {
    let client = reqwest::Client::new();

    let result = client
        .get(format!("https://{}/users/{}/orgs", conf.endpoint, name))
        .headers(Github::get_auth_header(conf.token.clone()))
        .send()
        .await
        .map_err(
            |e| {
                Error::new(ErrorKind::Other, e.to_string())
            }
        );

    match result {
        Ok(result) => {
            let status = result.status().as_u16();
            match status.clone() {
                200 => {
                    let transpilers: Vec<Transpiler> = result
                        .json().await
                        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
                    Ok(transpilers.iter().any(|t| t.login == name))
                },
                401 => {
                    Err(Error::new(ErrorKind::PermissionDenied, "Unauthorized"))
                },
                _ => {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!("Failed while fetching user type {}", status)
                    ))
                }
            }
        },
        Err(e) => Err(e)
    }
}