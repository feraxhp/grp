// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cformat;
use serde::Deserialize;
use serde_json::Value;
use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::platform::Platform;

pub enum UserType {
    Logged, // User that is logged in
    Organization, // Organization that belongs to the logged user
    Free, // User that is not logged in
}

#[derive(Deserialize)]
struct Transpiler {
    login: Option<String>,
    name: Option<String>,
}

impl Platform {
    pub fn get_org_name(&self, json: Value) -> String {
        match self {
            Platform::Github => json["login"].as_str().unwrap().to_string(),
            Platform::Gitea => json["name"].as_str().unwrap().to_string(),
            Platform::Gitlab => { panic!("GitLab doesn't support organizations"); }
        }
    }

    pub async fn get_user_type(&self, name: &str, conf: Config) -> Result<UserType, Error> {
        match (self.is_logged_user(name, conf.clone()).await, self) {
            (Ok(true), _) => Ok(UserType::Logged),
            (Ok(false), Platform::Github) |
            (Ok(false), Platform::Gitea) => {
                match self.is_organization(name, conf).await {
                    Ok(true) => Ok(UserType::Organization),
                    Ok(false) => Ok(UserType::Free),
                    Err(e) => Err(e),
                }
            }
            (Ok(false), Platform::Gitlab) => Ok(UserType::Free),
            (Err(e), _) => Err(e),
        }
    }

    pub async fn get_logged_user(&self, conf: Config) -> Result<String, Error> {
        let client = reqwest::Client::new();

        let result = client
            .get(format!("{}/user", self.get_base_url(conf.endpoint.clone())))
            .headers(self.get_auth_header(conf.token.clone()))
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

        let text = self.error_manager(
            result,
            DebugData {
                rtype: Rtype::UserList,
                owner: "".to_string(),
                repo: None,
            },
            conf.clone(),
            "Failed while fetching user type".to_string()
        ).await?;

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

        match &self {
            Platform::Github | Platform::Gitea => get_value_or_error(&transpiler.login, &text, "login"),
            Platform::Gitlab => get_value_or_error(&transpiler.name, &text, "name"),
        }
    }

    pub async fn is_logged_user(&self, name: &str, conf: Config) -> Result<bool, Error> {
        let user = self.get_logged_user(conf).await?;

        Ok(user == name)
    }

    pub async fn is_organization(&self, name: &str, conf: Config) -> Result<bool, Error> {
        let client = reqwest::Client::new();

        let result = client
            .get(format!("{}/user/orgs", self.get_base_url(conf.endpoint.clone())))
            .headers(self.get_auth_header(conf.token.clone()))
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

        let text = self.error_manager(
            result,
            DebugData {
                rtype: Rtype::UserList,
                owner: name.to_string(),
                repo: None,
            },
            conf.clone(),
            "Failed while fetching user type".to_string()
        ).await?;

        let transpilers: Vec<Value> = serde_json::from_str(&text.clone())
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

        Ok(transpilers.iter().any(|t| self.get_org_name(t.clone()) == name))
    }
}

fn get_value_or_error(option: &Option<String>, text: &String, att: &str) -> Result<String, Error> {
    option
        .as_ref()
        .map(|string| string.to_string())
        .ok_or_else(|| Error::new(
            ErrorType::Dezerialized,
            vec![
                format!("{} not found at:", att).as_str(),
                text
            ],
        ))
}