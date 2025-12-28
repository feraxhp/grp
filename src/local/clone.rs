use git2::{Error, Repository};
use indicatif::HumanBytes;
use std::path::PathBuf;
use git2::build::RepoBuilder;

use crate::animations::animation::Subprogress;
use crate::girep::animation::Animation;
use crate::girep::platform::Platform;
use crate::girep::config::Config;
use crate::girep::common::structs::Repo;
use crate::local::git::options::Methods;
use crate::local::git::structs::GitUtils;

#[derive(Clone, Debug)]
pub struct CloneOptions {
    pub path: PathBuf,
    pub branch: Option<String>,
    pub bare: bool,
}

impl Platform {
    pub async fn clone_repo<A: Subprogress + Animation + ?Sized>(&self,
        owner: &String, repo: &String,
        options: &CloneOptions,
        config: &Config,
        animation: &mut Box<A>
    ) -> Result<Repo, Error> {
        
        animation.change_message("Preparing clone ...");
        let url = self.generate_clone_url(&config.endpoint, &owner, &repo);
        
        Self::clone_by_url(&url, options, config, animation).await.map(|_| Repo {
            path: format!("{}/{}", owner, &repo),
            name: repo.clone(),
            private: None,
            url: options.path.as_os_str().to_str().unwrap_or("{{ Broken path }}").to_string(),
            git: url,
            description: None,
        })
    }
    
    pub async fn clone_by_url<A: Animation + Subprogress + ?Sized>(
        url: &String, 
        options: &CloneOptions,
        config: &Config,
        animation: &mut Box<A>
    ) -> Result<Repository, Error> { 
        animation.change_message("Setting up credentials ...");
        let mut callbacks = GitUtils::get_credential_callbacks(config);
        
        let _objects = animation.add();
        let _deltas = animation.add();
        
        callbacks.transfer_progress(|stats| {
            if stats.total_objects() == 0 { return true; } 
            else if stats.received_objects() == stats.total_objects() {
                animation.change_message("Resolving deltas ...");
                animation.set_state(1, stats.received_objects() as u64);
                animation.set_total(
                    2, stats.total_deltas() as u64, 
                    "    🔄 {percent:>3.blue}% {bar:30.green/blue}    {pos}/{len} on {elapsed_precise:.yellow}"
                );
                animation.set_state(2, stats.indexed_deltas() as u64);
            } 
            else {
                animation.change_message("Downloading objects ...");
                animation.set_total(
                    1, stats.total_objects() as u64, 
                    "    ⬇️ {percent:>3.blue}% {bar:30.green/blue}    {pos}/{len}: [{msg}] on {elapsed_precise:.yellow}"
                );
                animation.set_state(1, stats.received_objects() as u64);
                animation.set_message(1, format!("{} indexed: {}", HumanBytes(stats.received_bytes() as u64), stats.indexed_objects()));
            };
            
            true
        });
        
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        match options.branch.clone() {
            Some(value) => { builder.branch(value.as_str()); },
            None => { }
        };
        
        builder.remote_create(
            |r, _, url| r.remote(config.pconf.as_str(), url)
        );
        
        builder.bare(options.bare.clone());
        
        animation.change_message("Cloning repository ...");
        match builder.clone(url.as_str(), options.path.as_path()) {
            Ok(r) => {
                match GitUtils::get_branch_name(&r) {
                    Ok(b) => {
                        let _ = Methods::UPSTREAM
                            .set_upstream(&r, &b, config.pconf.as_str());
                    }
                    Err(_) => todo!(),
                };
                
                Ok(r)
            },
            Err(e) => Err(e),
        }
    }

    fn generate_clone_url<S: AsRef<str>>(&self, endpoint: &S, owner: &S, repo: &S) -> String {
        match self {
            Platform::Github => format!("https://github.com/{}/{}.git", owner.as_ref(), repo.as_ref()),
            Platform::Gitlab |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => format!("https://{}/{}/{}.git", endpoint.as_ref(), owner.as_ref(), repo.as_ref()),
        }
    }

}
