use std::path::PathBuf;

use git2::{Error, Repository};

use crate::girep::platform::Platform;


impl Platform {
    pub fn add_remote(&self, name: &String, url: &String, path: &PathBuf) -> Result<(), Error> {
        let repo = Repository::discover(path)?;
        let _ = repo.remote(name, url)?;
        Ok(())
    }
}
