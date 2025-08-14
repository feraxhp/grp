use git2::{Error, Repository};
use std::path::PathBuf;
use git2::build::RepoBuilder;

use crate::girep::animation::Animation;
use crate::girep::platform::Platform;
use crate::girep::config::Config;
use crate::girep::common::structs::Repo;
use crate::local::git::options::Methods;
use crate::local::git::structs::GitUtils;


impl Platform {
    pub async fn clone_repo<A: Animation + ?Sized>(&self,
        owner: &String, repo: &String,
        path: &PathBuf, 
        branch: Option<String>,
        config: &Config,
        animation: Option<&Box<A>>
    ) -> Result<Repo, Error> {
        
        if let Some(an) = animation { an.change_message("Preparing clone ..."); }
        let url = self.generate_clone_url(&config.endpoint, &owner, &repo);
        
        Self::clone_by_url(&url, path, branch, config, animation).await.map(|_| Repo {
            path: format!("{}/{}", owner, &repo),
            name: repo.clone(),
            private: None,
            url: path.as_os_str().to_str().unwrap_or("{{ Broken path }}").to_string(),
            git: url,
            description: None,
        })
    }
    
    pub async fn clone_by_url<A: Animation + ?Sized>(
        url: &String, 
        path: &PathBuf, 
        branch: Option<String>,
        config: &Config,
        animation: Option<&Box<A>>
    ) -> Result<Repository, Error> {        
        if let Some(an) = animation { an.change_message("Setting up credentials ..."); }
        let mut callbacks = GitUtils::get_credential_callbacks(config);
        
        if let Some(an) = animation {
            callbacks.transfer_progress(|stats| {
                let message = if stats.total_objects() == 0 { return true; } 
                else if stats.received_objects() == stats.total_objects() {
                    format!(
                        "Resolving deltas {}/{}",
                        stats.indexed_deltas(),
                        stats.total_deltas()
                    )
                } 
                else {
                    format!(
                        "Received {}/{} objects ({}) in {} bytes",
                        stats.received_objects(),
                        stats.total_objects(),
                        stats.indexed_objects(),
                        stats.received_bytes()
                    )
                };
                
                an.change_message(message);
                true
            });
        }
        
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        match branch {
            Some(value) => { builder.branch(value.as_str()); },
            None => { }
        };
        
        builder.remote_create(
            |r, _, url| r.remote(config.pconf.as_str(), url)
        );
        
        if let Some(an) = animation { an.change_message("Cloning repository ..."); }
        match builder.clone(url.as_str(), path.as_path()) {
            Ok(r) => {
                let branch = GitUtils::get_branch_name(&r)?;
                let _ = Methods::UPSTREAM.set_upstream(&r, &branch, config.pconf.as_str());
                Ok(r)
            },
            Err(e) => Err(e),
        }
    }

    fn generate_clone_url<S: AsRef<str>>(&self, endpoint: &S, owner: &S, repo: &S) -> String {
        match self {
            Platform::Github => format!("https://github.com/{}/{}.git", owner.as_ref(), repo.as_ref()),
            Platform::Gitlab |
            Platform::Gitea => format!("https://{}/{}/{}.git", endpoint.as_ref(), owner.as_ref(), repo.as_ref()),
        }
    }

}
