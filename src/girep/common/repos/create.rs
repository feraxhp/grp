// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::animations;
use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::repo::Repo;
use crate::girep::common::repos::list::Transpiler;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::platform::Platform;
use crate::girep::gitea::errors::error_mannager;
use color_print::cformat;
use std::process::exit;

impl Platform {
    pub async fn create_repo(&self, owner: String, repo: Repo, config: Config) -> Result<Repo, Error> {
        let client = reqwest::Client::new();

        let load_animation = animations::creation::Create::new("Creating repository ...");

        let url = match self.get_user_type(owner.as_str(), config.clone()).await {
            Ok(user) => self.url_create_repo(owner.clone(), user, config.endpoint.clone()),
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                e.show();
                exit(101);
            }
        };

        let json = serde_json::json!({
            "name": repo.full_name,
            "description": repo.description,
            "private": repo.state == "private",
        });

        let result = match client.post(url).headers(self.get_auth_header(config.token.clone()))
            .header("content-type", "application/json").json(&json).send().await {
                Ok(response) => response,
                Err(e) => {
                    load_animation.finish_with_error("Failed to contact the platform");
                    return Err(
                        Error::new(
                            ErrorType::Unknown,
                            vec![
                                e.to_string().as_str(),
                            ]
                        )
                    )
                }
            };

        let response_text = error_mannager(
            result,
            DebugData{
                rtype: Rtype::Create,
                owner: owner.clone(),
                repo: Some(repo.full_name.clone()),
            },
            config.clone(),
            "Failed to create repository".to_string(),
        ).await;

        let response_text = match response_text {
            Ok(text) => text,
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                return Err(e);
            }
        };

        let transpiler: Transpiler = match serde_json::from_str(&response_text.clone()) {
            Ok(repos) => repos,
            Err(e) => {
                load_animation.finish_with_error("Failed to create repository");
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

        load_animation.finish_with_success("Done!");

        Ok(
            Repo {
                full_name: transpiler.full_name,
                description: transpiler.description.unwrap_or_default(),
                state: if transpiler.private { "private".to_string() } else { "public".to_string() },
                html_url: transpiler.html_url,
                clone_url: transpiler.clone_url,
            }
        )
    }
}