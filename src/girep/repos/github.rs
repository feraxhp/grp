// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::process::exit;
use async_trait::async_trait;
use color_print::cprintln;
use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repo::Repo;
use hyper::HeaderMap;
use serde::Deserialize;
use crate::animations;

#[derive(Deserialize)]
struct Transpiler {
    full_name: String,
    description: String,
    private: bool,
    html_url: String,
    clone_url: String,
}

#[derive(Deserialize)]
struct Error {
    message: String,
    status: String,
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

        let response_text = result.text().await.unwrap_or_else(|e| {
            load_animation.finish_with_error("Failed to fetch repositories");
            eprintln!("Failed to read the response text: {}", e);
            cprintln!("<y>Unknown error</>");
            exit(101);
        });

        let repositories: Vec<Transpiler> = match serde_json::from_str(&response_text) {
            Ok(transpilers) => transpilers,
            Err(_) => {
                let error: Error = match serde_json::from_str(&response_text) {
                    Ok(error) => error,
                    Err(_) => {
                        Error {
                            status: "101".to_string(),
                            message: format!("{}", &response_text)
                        }
                    }
                };
                match error.message.as_str() {
                    "Bad credentials" => {
                        load_animation.finish_with_error("Bad credentials");
                        eprintln!("* Please check your token.");
                        eprintln!("  Pconf name: {}", &self.config.pconf.clone());
                        cprintln!("  Platform: <b,i,u>{}</>", self.config.endpoint.clone());
                        eprintln!("  User: {}", owner);
                    },
                    "Not Found" => {
                        load_animation.finish_with_error("User/org does not exist");
                        cprintln!("Platform: <b,i,u>{}</>", self.config.endpoint.clone());
                        cprintln!("User/org: <m>({})</>", owner);
                    },
                    _ => {
                        load_animation.finish_with_error("Unknown error");
                        println!("{}", &response_text);
                    }
                }
                exit(101);
            }
        };

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
}
