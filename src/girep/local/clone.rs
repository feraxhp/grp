// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::path::PathBuf;
use git2::build::RepoBuilder;
use git2::{Cred, RemoteCallbacks};
use crate::animations::animation::Animation;
use crate::animations::process::Process;
use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::repo::Repo;
use crate::girep::platform::Platform;

impl Platform {
    pub async fn clone_repo(&self,
        owner: String,
        repo: String,
        path: PathBuf,
        branch: Option<String>,
        conf: Config
    ) -> Result<Repo, Error> {

        let animation = Process::new("Cloning repository ...");

        let url = self.generate_clone_url(conf.endpoint.clone(), owner.clone(), repo.clone());

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(GitUtils::get_credential_callbacks(conf.clone()));

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);
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
                let e = Error::git_to_local(
                    e,
                    path.clone(),
                    conf.clone()
                );

                animation.finish_with_error(e.message.as_str());

                Err(e)
            }
        }
    }

    fn generate_clone_url(&self, endpoint: String, owner: String, repo: String) -> String {
        match self {
            Platform::Github => format!("https://github.com/{}/{}.git", owner, repo),
            Platform::Gitea => format!("https://{}/{}/{}.git", endpoint, owner, repo),
        }
    }

}
