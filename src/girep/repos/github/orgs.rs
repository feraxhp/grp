// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repos::github::implementation::Github;
use serde::Deserialize;
use std::io::{Error, ErrorKind};

#[derive(Deserialize)]
struct Transpiler {
    login: String,
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