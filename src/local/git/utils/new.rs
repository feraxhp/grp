use std::path::Path;

use git2::Repository;
use mynox::Error;

use crate::{errors::ToError, local::{git::structs::Action, structs::Git2Context}};

use super::GitUtils;

impl GitUtils {
    pub fn from(repo: Repository) -> Self{ Self { git2: repo } }
    
    pub fn discover(path: impl AsRef<Path>) -> Result<Self, Error>{
        let repo = Repository::discover(&path);
        
        match repo {
            Ok(git2) => Ok(Self { git2 }),
            Err(e) => {
                let path = path.as_ref().display().to_string();
                let err = e.to_error(Git2Context { 
                    action: Action::Discover,
                    owner: None, repo: path.as_str(),
                    config: None, usettings: None,
                });
                
                Err(err)
            },
        }
    }
}
