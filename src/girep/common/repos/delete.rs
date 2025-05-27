// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::animations::animation::Animation;
use crate::girep::animations::delition::Delete;
use crate::girep::config::config::Config;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::platform::Platform;

impl Platform {
    pub async fn delete_repo(&self, owner: String, repo: String, config: Config) -> Result<(), Error> {
        let client = reqwest::Client::new();

        let load_animation = Delete::new("Deleting repository ...");

        let url = self.url_delete_repo(owner.clone(), repo.clone(), config.endpoint.clone());

        let result = match client.delete(url)
            .headers(self.get_auth_header(config.token.clone())).send().await {
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

        if result.status().as_u16() == 204 {
            load_animation.finish_with_success("Done!");
            return Ok(());
        }

        Err(
            self.error_manager(
                result,
                DebugData{
                    rtype: Rtype::Delete,
                    owner: owner.clone(),
                    repo: Some(repo.clone()),
                },
                config.clone(),
                "Failed to delete repository".to_string(),
            ).await.unwrap_err()
        )
    }
}