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
    pub(crate) fn clone_repo(&self, owner: String, repo: String, path: PathBuf, branch: Option<String>, conf: Config) -> Result<Repo, Error> {

        let animation = Process::new("Cloning repository ...");

        let url = self.url_clone_repo(owner.clone(), repo.clone(), conf.endpoint.clone()) + ".git";

        // let mut callbacks = RemoteCallbacks::new();
        // callbacks.credentials(|_, _, _| {
        //     Cred::userpass_plaintext("oauth2", conf.token.as_str())
        // });
        //
        // let mut fo = git2::FetchOptions::new();
        // fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        // builder.fetch_options(fo);
        match branch {
            Some(value) => { builder.branch(value.as_str()); },
            None => { }
        };

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
