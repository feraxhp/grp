// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::animations;
use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repo::Repo;
use crate::girep::repos::github::errors::error_mannager;
use async_trait::async_trait;
use color_print::cprintln;
use hyper::HeaderMap;
use serde::Deserialize;
use std::process::exit;

#[derive(Deserialize)]
struct Transpiler {
    full_name: String,
    description: Option<String>,
    private: bool,
    html_url: String,
    clone_url: String,
}

pub(crate) struct Github {
    config: Config,
    header: HeaderMap,
}

#[async_trait]
impl Platform for Github {
    fn new(config: Config) -> Self {
        let header = Self::get_auth_header(config.token.clone());
        Github { config, header }
    }

    fn get_auth_header(token: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "reqwest > rust > grp".parse().unwrap());
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        headers
    }
    async fn list_repos(&self, owner: Option<String>) -> Vec<Repo> {

        let owner = owner.unwrap_or(self.config.user.clone());

        let load_animation = animations::fetch::Fetch::new("Fetching repositories ...");

        let client = reqwest::Client::new();
        let result = client
            .get(format!("https://{}/users/{}/repos", self.config.endpoint, owner))
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
            owner,
            self.config.clone(),
            "Failed to fetch repositories".to_string(),
            |str| { load_animation.finish_with_error(str); }
        ).await;

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
                        description: transpiler.description.unwrap_or("".to_string()),
                        state: if transpiler.private { "private".to_string() } else { "public".to_string() },
                        html_url: transpiler.html_url,
                        clone_url: transpiler.clone_url,
                    }
            )
        .collect()
    }

    async fn create_repo(&self, owner: String, repo: Repo) -> Repo {
        todo!()
    }
}