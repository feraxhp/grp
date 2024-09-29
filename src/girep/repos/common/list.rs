// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::animations;
use crate::girep::config::Config;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::repo::Repo;
use crate::girep::repos::common::paggination::paggination_mannager;
use crate::girep::repos::common::structs::{DebugData, Rtype};
use crate::girep::platform::Platform;
use crate::girep::repos::github::errors::error_mannager;
use futures::future::join_all;
use serde::Deserialize;
use std::process::exit;

#[derive(Deserialize)]
pub(crate) struct Transpiler {
    pub(crate) full_name: String,
    pub(crate) description: Option<String>,
    pub(crate) private: bool,
    pub(crate) html_url: String,
    pub(crate) clone_url: String,
}

impl Platform {
    pub(crate) async fn list_repos(&self, owner: Option<String>, config: Config) -> (Vec<Repo>, Vec<Error>) {
        let header_map = self.get_auth_header(config.token.clone());
        let owner = owner.unwrap_or(config.user.clone());

        let load_animation = animations::fetch::Fetch::new("Fetching repositories ...");


        let url = match self.get_user_type(owner.as_str(), config.clone()).await {
            Ok(user) => self.url_list_repos(owner.clone(), user, config.endpoint.clone()),
            Err(e) => {
                load_animation.finish_with_error(e.message.as_str());
                e.show();
                exit(101);
            }
        };

        let (responses,mut erros) = paggination_mannager(url, header_map).await;

        let responses: Vec<_> = responses.into_iter().map(|response| {
            error_mannager(
                response,
                DebugData{
                    rtype: Rtype::List,
                    owner: owner.clone(),
                    repo: None,
                },
                config.clone(),
                "Failed to fetch repositories".to_string(),
            )
        }).collect();

        let repos = join_all(responses).await;

        let (repos, repos_erros): (Vec<_>, Vec<_>) = repos.into_iter().partition(Result::is_ok);

        let repos_erros: Vec<Error> = repos_erros.into_iter().map(Result::unwrap_err).collect();

        erros.extend(repos_erros);

        let mut repositories_transpiler: Vec<Transpiler> = Vec::new();
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
            repositories_transpiler.extend(repository);
        }

        if erros.is_empty() { load_animation.finish_with_success("Repositories fetched successfully!"); }
        else {
            load_animation.finish_with_warning("Some repositories might be missing");
        }

        // Return the list of repositories
        let repos = repositories_transpiler
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
            .collect();

        (repos, erros)
    }
}