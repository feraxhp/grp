use std::path::PathBuf;
use git2::{Error, Repository};

use crate::local::{git::{options::Methods, structs::GitUtils}, structs::Local};

impl Local {
    pub fn add_remote(&self, name: &str, url: &str, path: &PathBuf) -> Result<(), Error> {
        let repo = Repository::discover(path)?;
        let _ = repo.remote(name, url)?;
        Ok(())
    }
    pub fn set_upstream_to_local_branch(&self, name: &str, path: &PathBuf) -> Result<String, Error> {
        let repo = Repository::discover(path)?;
        let branch = GitUtils::get_branch_name(&repo)?;
        Methods::Upstream.set_upstream(&repo, &branch, name)?;
        Ok(branch)
    }
}
