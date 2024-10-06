// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cformat;
use crate::animations;
use crate::animations::animation::Animation;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::common::orgs::org::Org;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::config::Config;
use crate::girep::platform::Platform;

impl Platform {
    pub(crate) async fn create_org(&self, name: String, config: Config) -> Result<Org, Error>{
        let header_map = self.get_auth_header(config.token.clone());

        let url = self.url_create_org(config.endpoint.clone());

        let load_animation = animations::creation::Create::new("Creating organization ...");

        if matches!(self, Platform::Github) {
            load_animation.finish_with_warning("Github API Error");
            return Err(
                Error::new_custom(
                    "Github API Error".to_string(),
                    vec![
                        cformat!("* The GitHub api does not allow <r,i>Create Orgs</>"),
                        "  You can do it by following this link:".to_string(),
                        cformat!("  → <b,i,u>https://github.com/organizations/plan</>"),
                    ]
                )
            )
        };

        let json = serde_json::json!({
            "username": name,
        });
        let client = reqwest::Client::new();

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

        let response_text = self.error_manager(
            result,
            DebugData {
                rtype: Rtype::CreateOrg,
                owner: name.clone(),
                repo: None,
            },
            config.clone(),
            "Failed to create Organization".to_string(),
        ).await;

        let response_text = match response_text {
            Ok(text) => text,
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                return Err(e);
            }
        };

        let response: Org = match serde_json::from_str(&response_text.clone()) {
            Ok(org) => org,
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

        load_animation.finish_with_success("Organization created successfully!");

        Ok(response)
    }
}