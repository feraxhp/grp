// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::animations;
use crate::animations::delition::Delete;
use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::repo::Repo;
use crate::girep::repos::comond::structs::{DebugData, Rtype};
use crate::girep::repos::github::errors::error_mannager;
use crate::girep::repos::github::user::{get_user_type, is_logged_user};
use crate::girep::repos::paggination::paggination_mannager;
use crate::girep::repos::user_type::UserType;
use async_trait::async_trait;
use color_print::cprintln;
use futures::future::join_all;
use futures::SinkExt;
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

        let url = match get_user_type(owner.as_str(), self.config.clone()).await {
            Ok(UserType::Logged) => format!("https://{}/user/repos", self.config.endpoint),
            Ok(UserType::Organization) => format!("https://{}/orgs/{}/repos", self.config.endpoint, owner),
            Ok(UserType::Free) => format!("https://{}/users/{}/repos", self.config.endpoint, owner),
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                e.show();
                exit(101);
            }
        };

        let (responses,mut erros) = paggination_mannager(
            url,
            self.header.clone()
        ).await;

        let responses: Vec<_> = responses.into_iter().map(|response| {
            error_mannager(
                response,
                DebugData{
                    rtype: Rtype::List,
                    owner: owner.clone(),
                    repo: None,
                },
                self.config.clone(),
                "Failed to fetch repositories".to_string(),
            )
        }).collect();

        let repos = join_all(responses).await;

        let (repos, repos_erros): (Vec<_>, Vec<_>) = repos.into_iter().partition(Result::is_ok);

        let repos_erros: Vec<Error> = repos_erros.into_iter().map(Result::unwrap_err).collect();

        erros.extend(repos_erros);

        let mut repositories_transpilet: Vec<Transpiler> = Vec::new();
        for repo in repos {
            let repo = match repo {
                Ok(repo) => repo,
                Err(e) => {
                    erros.push(e);
                    continue;
                }
            };
            let repository: Vec<Transpiler> = match serde_json::from_str(&repo.clone()) {
                Ok(repos) => repos,
                Err(e) => {
                    erros.push(Error::new(
                        ErrorType::Dezerialized,
                        vec![
                            e.to_string().as_str(),
                            repo.as_str()
                        ]
                    ));
                    continue;
                }
            };
            repositories_transpilet.extend(repository);
        }

        if erros.len() > 0 {
            load_animation.finish_with_warning("Some repositories might be missing");
            for error in erros {
                error.show();
            }
        } else {
            load_animation.finish_with_success("Done!");
        }

        // Return the list of repositories
        repositories_transpilet
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
        let load_animation = animations::creation::Create::new("Creating repository ...");

        let url = match is_logged_user(owner.as_str(), self.config.clone()).await {
            Ok(true) => format!("https://{}/user/repos", self.config.endpoint),
            Ok(false) => format!("https://{}/orgs/{}/repos", self.config.endpoint, owner),
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                e.show();
                exit(101);
            }
        };

        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .headers(self.header.clone())
            .json(&serde_json::json!({
                "name": repo.full_name,
                "description": repo.description,
                "private": repo.state == "private"
            }))
            .send()
            .await
            .unwrap_or_else(
                |e| {
                    load_animation.finish_with_error("Failed to create repository");
                    cprintln!("<r>*</> {}", e);
                    cprintln!("<y>  Please verify your endpoint</>");
                    exit(101);
                }
        );

        let response_text = error_mannager(
            result,
            DebugData{
                rtype: Rtype::Create,
                owner: owner.clone(),
                repo: Some(repo.full_name.clone()),
            },
            self.config.clone(),
            "Failed to create repository".to_string(),
        ).await;

        let response_text = match response_text {
            Ok(text) => text,
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                e.show();
                exit(101);
            }
        };

        let transpiler: Transpiler = serde_json::from_str(&response_text)
            .unwrap_or_else(|e| {
                load_animation.finish_with_error("Failed to create repository");
                eprintln!("* Failed to parse the response: {}", e);
                eprintln!("  Response: {}", response_text);
                cprintln!("<y>* Unknown error</>");
                exit(101);
            });

        load_animation.finish_with_success("Done!");

        Repo {
            full_name: transpiler.full_name,
            description: transpiler.description.unwrap_or("".to_string()),
            state: if transpiler.private { "private".to_string() } else { "public".to_string() },
            html_url: transpiler.html_url,
            clone_url: transpiler.clone_url,
        }
    }

    async fn delete_repo(&self, owner: String, repo: String) -> bool {
        let client = reqwest::Client::new();

        let load_animation = Delete::new("Deleting repository ...");

        let url = format!("https://{}/repos/{}/{}", self.config.endpoint, owner, repo);

        let result = client
            .delete(url)
            .headers(self.header.clone())
            .send()
            .await
            .unwrap_or_else(
                |e| {
                    load_animation.finish_with_error("Failed to contact the platform");
                    eprintln!("* Failed to delete repository: {}", e);
                    cprintln!("<y>  Unknown error</>");
                    exit(101);
                }
            );

        if result.status().as_u16() == 204 {
            load_animation.finish_with_success("Done!");
            return true;
        }

        let _response_text = error_mannager(
            result,
            DebugData{
                rtype: Rtype::Delete,
                owner: owner.clone(),
                repo: Some(repo.clone()),
            },
            self.config.clone(),
            "Failed to delete repository".to_string(),
        ).await;

        false
    }
}