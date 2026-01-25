use crate::error::structs::Error;
use crate::json::JSON;
use crate::platform::Platform;
use crate::common::structs::Repo;
use crate::specific::{gitea, github, gitlab};


impl Repo {
    pub fn as_json(&self, platform: &Platform) -> serde_json::Value {
        match platform {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                serde_json::json!({
                    "name": self.name,
                    "description": self.description,
                    "private": self.private.unwrap_or(false),
                })
            }
            Platform::Gitlab => {
                serde_json::json!({
                    "path": self.name,
                    "description": self.description,
                    "visibility": if self.private.unwrap_or(false) { "private" } else { "public" },
                })
            }
        }
    }
    
    pub fn from_text(text: &String, platform: &Platform) -> Result<Self, Error> {
        let repo = match platform {
            Platform::Github => { 
                let repo: github::parser::Repository = JSON::from_str(text)?;
                
                Repo {
                    name: repo.name.clone(),
                    path:  repo.full_name.clone(),
                    private: Some(repo.private),
                    url:   repo.html_url.clone(),
                    git: repo.clone_url.clone(),
                    description: repo.description.clone(),
                }
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { 
                let repo: gitea::parser::Repository = JSON::from_str(text)?;
                
                Repo {
                    name: repo.name.clone(),
                    path:  repo.full_name.clone(),
                    private: Some(repo.private),
                    url:   repo.html_url.clone(),
                    git: repo.clone_url.clone(),
                    description: repo.description.clone(),
                }
            },
            Platform::Gitlab => { 
                let repo: gitlab::parser::Repository = JSON::from_str(text)?;
                
                Repo {
                    name: repo.path.clone(),
                    path:  repo.path_with_namespace.clone(),
                    private: Some(repo.visibility == "private"),
                    url:   repo.web_url.clone(),
                    git:   repo.http_url_to_repo.clone(),
                    description: repo.description.clone(),
                }
            },
        };
        
        Ok(repo)
    }
    
    pub fn from_text_array(text: &String, platform: &Platform) -> Result<Vec<Self>, Error> {
        let repos = match platform {
            Platform::Github => {
                let tmp: Vec<github::parser::Repository> = JSON::from_str(text)?;
                
                let repos = tmp.iter().map(|repo| {
                    Repo {
                        name: repo.name.clone(),
                        path:  repo.full_name.clone(),
                        private: Some(repo.private),
                        url:   repo.html_url.clone(),
                        git:   repo.clone_url.clone(),
                        description: repo.description.clone(),
                    }
                }).collect();
                
                repos
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let tmp: Vec<gitea::parser::Repository> = JSON::from_str(text)?;
                
                let repos = tmp.iter().map(|repo| {
                    Repo {
                        name: repo.name.clone(),
                        path:  repo.full_name.clone(),
                        private: Some(repo.private),
                        url:   repo.html_url.clone(),
                        git:   repo.clone_url.clone(),
                        description: repo.description.clone(),
                    }
                }).collect();
                
                repos
            },
            Platform::Gitlab => {
                let tmp: Vec<gitlab::parser::Repository> = JSON::from_str(text)?;
                
                let repos = tmp.iter().map(|repo| {
                    Repo {
                        name: repo.path.clone(),
                        path:  repo.path_with_namespace.clone(),
                        private: Some(repo.visibility == "private"),
                        url:   repo.web_url.clone(),
                        git:   repo.http_url_to_repo.clone(),
                        description: repo.description.clone(),
                    }
                }).collect();
                
                repos
            }
        };
        
        Ok(repos)
    }
}