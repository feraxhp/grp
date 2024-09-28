// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::path::PathBuf;
use git2::build::RepoBuilder;
use git2::{Cred, RemoteCallbacks};
use crate::animations::process::Process;
use crate::girep::config::Config;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::repo::Repo;
use crate::girep::repos::common::supported::Platform;

impl Platform {
    pub(crate) async fn clone_repo(&self,
        owner: String,
        repo: String,
        path: PathBuf,
        branch: Option<String>,
        conf: Config
    ) -> Result<Repo, Error> {

        let (repos, erros) = self.list_repos(Some(owner.clone()), conf.clone()).await;

        let animation = Process::new("Cloning repository ...");

        let full_name = format!("{}/{}", owner.clone(), repo.clone());
        let url = match repos.iter().find(|r| { r.full_name == full_name.clone() }).clone() {
            Some(repo) => repo.clone_url.clone(),
            None => {
                let additional_info: Vec<Vec<String>> = erros.iter().map(|e| {
                    e.content.clone()
                }).collect();

                let mut additional_info: Vec<&str> = additional_info.iter()
                    .flat_map(|inner_vec| inner_vec.iter().map(|s| s.as_str()))
                    .collect();

                let mut full_name = vec![full_name.as_str()];

                full_name.append(&mut additional_info.clone());

                let error = Error::new(ErrorType::NotFoundRepo, full_name);

                animation.finish_with_error(error.message.as_str());

                return Err(error)
            }
        };

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_, _, _| {
            Cred::userpass_plaintext("oauth2", conf.token.as_str())
        });

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);
        match branch {
            Some(value) => { builder.branch(value.as_str()); },
            None => { }
        };

        eprintln!("{}", url.as_str());
        match builder.clone(url.as_str(), path.as_path()) {
            Ok(repostory) => {
                animation.finish_with_success("Repository cloned successfully");
                Ok(
                    Repo {
                        full_name: format!("{}/{}", owner, repo),
                        description: "".to_string(),
                        state: "Local".to_string(),
                        html_url: "".to_string(),
                        clone_url: url,
                    }
                )
            },
            Err(e) => {
                animation.finish_with_error("Failed to clone the repository");

                Err(
                    Error::new(
                        ErrorType::Unknown,
                        vec![
                            e.to_string().as_str(),
                            e.message().clone()
                        ]
                    )
                )
            }
        }
    }
}
