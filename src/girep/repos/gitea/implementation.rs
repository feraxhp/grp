// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::animations;
use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repo::Repo;
use crate::girep::repos::gitea::errors::{error_mannager, DebugData};
use async_trait::async_trait;
use color_print::cprintln;
use hyper::HeaderMap;
use serde::Deserialize;
use std::process::exit;

#[derive(Deserialize)]
struct Transpiler {
    full_name: String,
    description: String,
    private: bool,
    html_url: String,
    clone_url: String,
}

pub(crate) struct Gitea {
    config: Config,
    header: HeaderMap,
}

#[async_trait]
impl Platform for Gitea {
    fn new(config: Config) -> Self {
        let header = Self::get_auth_header(config.token.clone());
        Gitea { config, header }
    }

    fn get_auth_header(token: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("content-type.rs", "application/json".parse().unwrap());
        headers.insert("authorization", format!("Bearer {}", token).parse().unwrap());

        headers
    }
    async fn list_repos(&self, owner: Option<String>) -> Vec<Repo> {
        let owner = owner.unwrap_or(self.config.user.clone());
        let client = reqwest::Client::new();

        let load_animation = animations::fetch::Fetch::new("Fetching repositories ...");

        let result = client
            .get(format!("https://{}/api/v1/users/{}/repos", self.config.endpoint, owner))
            .headers(self.header.clone())
            .send()
            .await
            .unwrap_or_else(
                |e| {
                    load_animation.finish_with_error("Failed to fetch repositories");
                    cprintln!("<r>*</> {}", e);
                    cprintln!("<y>Please verify your endpoint</>");
                    exit(101);
                }
            );

        let response_text = error_mannager(
                result,
                DebugData{
                    rtype: crate::girep::repos::gitea::errors::Rtype::List,
                    owner: owner.clone(),
                    repo: None,
                },
                self.config.clone(),
                "Failed to fetch repositories".to_string(),
                |str| { load_animation.finish_with_error(str); }
            )
            .await;

        let repositories: Vec<Transpiler> = serde_json::from_str(&response_text)
            .unwrap_or_else(|e| {
                load_animation.finish_with_error("Failed to fetch repositories");
                eprintln!("Failed to parse the response: {}", e);
                eprintln!("Response: {}", response_text);
                cprintln!("<y>Unknown error</>");
                exit(101);
            });

        load_animation.finish_with_success("Done!");

        // Return the list of repositories
        repositories
            .into_iter()
            .map(
                |transpiler|
                    Repo {
                        full_name: transpiler.full_name,
                        description: transpiler.description,
                        state: if transpiler.private { "private".to_string() } else { "public".to_string() },
                        html_url: transpiler.html_url,
                        clone_url: transpiler.clone_url,
                    }
            )
        .collect()
    }

    async fn create_repo(&self, owner: String, repo: Repo) -> Repo {
        let client = reqwest::Client::new();

        let load_animation = animations::creation::Create::new("Creating repository ...");

        let result = client
            .post(format!("https://{}/api/v1/user/repos", self.config.endpoint))
            .headers(self.header.clone())
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "name": repo.full_name,
                "description": repo.description,
                "private": repo.state == "private",
            }))
            .send()
            .await
            .unwrap_or_else(
                |e| {
                    load_animation.finish_with_error("Failed to contact the platform");
                    eprintln!("Failed to create repository: {}", e);
                    cprintln!("<y>Unknown error</>");
                    exit(101);
                }
            );

        let response_text = error_mannager(
                result,
                DebugData{
                    rtype: crate::girep::repos::gitea::errors::Rtype::Create,
                    owner: owner.clone(),
                    repo: Some(repo.full_name.clone()),
                },
                self.config.clone(),
                "Failed to create repository".to_string(),
                |str| { load_animation.finish_with_error(str); }
            ).await;

        let transpiler: Transpiler = serde_json::from_str(&response_text).unwrap_or_else(
            |e| {
                load_animation.finish_with_error("Failed to create repository");
                eprintln!("Failed to parse the response: {}", e);
                eprintln!("Response: {}", response_text);
                cprintln!("<y>Unknown error</>");
                exit(101);
            }
        );

        load_animation.finish_with_success("Done!");

        Repo {
            full_name: transpiler.full_name,
            description: transpiler.description,
            state: if transpiler.private { "private".to_string() } else { "public".to_string() },
            html_url: transpiler.html_url,
            clone_url: transpiler.clone_url,
        }
    }
}