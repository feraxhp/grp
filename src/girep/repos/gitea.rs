// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::process::exit;
use crate::girep::base::RepoProvider;
use crate::girep::config::Config;
use crate::girep::repo::Repo;
use hyper::HeaderMap;
use serde::Deserialize;

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
}

pub(crate) struct Gitea {
    config: Config,
    header: HeaderMap,
}

impl RepoProvider for Gitea {
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

        let result = client
            .get(format!("https://{}/api/v1/users/{}/repos", self.config.endpoint, owner))
            .headers(self.header.clone())
            .send()
            .await
            .unwrap();


        let response_text = result.text().await.unwrap_or_else(|e| {
            eprintln!("Failed to read the response text: {:?}", e);
            eprintln!("Unknown error");
            exit(101);
        });

        let repositories: Vec<Transpiler> = match serde_json::from_str(&response_text) {
            Ok(transpilers) => transpilers,
            Err(_) => {

                let error: Error = match serde_json::from_str(&response_text) {
                    Ok(error) => error,
                    Err(_) => {
                        Error {
                            message: format!("{}", response_text)
                        }
                    }
                };

                match error.message.as_str() {
                    "user does not exist [uid: 0, name: ]" => {
                        eprintln!("The token you provide could not be correct.");
                        eprintln!("Please check your token.");
                    },
                    _ => { println!("{}", &response_text); }
                }

                exit(101);
            }
        };

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