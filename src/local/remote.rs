use std::path::PathBuf;

use crate::local::git::structs::GitUtils;
use crate::local::git::options::Methods;
use crate::local::structs::LocalError;
use crate::local::structs::Local;


impl Local {
    pub fn add_remote(&self, name: &str, url: &str, path: &PathBuf) -> Result<(), LocalError> {
        let repo = GitUtils::discover(path)?;
        let _ = repo.remote(name, url)?;
        Ok(())
    }
    pub fn set_upstream_to_local_branch(&self, name: &str, path: &PathBuf) -> Result<String, LocalError> {
        let repo = GitUtils::discover(path)?;
        let branch = GitUtils::get_branch_name(&repo)?;
        Methods::Upstream.set_upstream(&repo, &branch, name)?;
        Ok(branch)
    }
}
