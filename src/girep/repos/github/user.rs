// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cformat;
use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repos::github::implementation::Github;
use serde::Deserialize;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
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
                Error::new(
                    ErrorType::Unknown,
                    vec![
                        cformat!("Failed while fetching user type").as_str(),
                        e.to_string().as_str()
                    ]
                )
            }
        )?;

    match result.status().as_u16() {
        200 => {
            let text = result.text().await.unwrap();
            let transpiler: Transpiler = serde_json::from_str(&text.clone())
                .map_err(
                    |e|
                        Error::new(
                            ErrorType::Dezerialized,
                        vec![
                            e.to_string().as_str(),
                            text.as_str()
                        ]
                    )
                )?;
            Ok(transpiler.login == name)
        },
        401 => {
            Err(Error::new(ErrorType::Unauthorized, vec![
                conf.pconf.as_str(),
                name.clone()
            ]))
        },
        _ => {
            Err(Error::new(
                ErrorType::Unknown,
                vec![
                    cformat!("Failed while fetching user type").as_str(),
                    result.status().as_u16().to_string().as_str()
                ]
            ))
        }
    }

}

async fn is_organization(name: &str, conf: Config) -> Result<bool, Error> {
    let client = reqwest::Client::new();

    let result = client
        .get(format!("https://{}/api/v1/user/orgs", conf.endpoint))
        .headers(Github::get_auth_header(conf.token.clone()))
        .send()
        .await
        .map_err(
            |e| {
                Error::new(
                    ErrorType::Unknown,
                    vec![
                        cformat!("Failed while fetching user type").as_str(),
                        e.to_string().as_str()
                    ]
                )
            }
        );

    match result {
        Ok(result) => {
            let status = result.status().as_u16();
            match status.clone() {
                200 => {
                    let text = result.text().await.unwrap();
                    let transpilers: Vec<Transpiler> = serde_json::from_str(&text.clone())
                        .map_err(
                            |e|
                                Error::new(
                                    ErrorType::Dezerialized,
                                    vec![
                                        e.to_string().as_str(),
                                        text.as_str()
                                    ]
                                )
                        )?;
                    Ok(transpilers.iter().any(|t| t.login == name))
                },
                401 => {
                    Err(Error::new(ErrorType::Unauthorized, vec![
                        conf.pconf.as_str(),
                        name.clone()
                    ]))
                },
                _ => {
                    Err(Error::new(
                        ErrorType::Unknown,
                        vec![
                            cformat!("Failed while fetching user type").as_str(),
                            status.to_string().as_str()
                        ]
                    ))
                }
            }
        },
        Err(e) => Err(e)
    }
}