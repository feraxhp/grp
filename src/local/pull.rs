use std::path::PathBuf;

use git2::{Error, Repository};

use crate::usettings::structs::{Pconf, Usettings};
use crate::local::git::options::Options;
use crate::girep::platform::Platform;
use crate::girep::animation::Animation;


#[allow(dead_code)]
impl Platform {
    pub(crate) fn pull_repo<A: Animation + ?Sized>(
        path: &PathBuf, 
        options: Options,
        pconf: Option<Pconf>, 
        usettings: &Usettings, 
        animation: Option<&Box<A>>
    ) -> Result<(Vec<String>, bool), Error> {
        if let Some(an) = animation { an.change_message("Getting the local repository ..."); }
        let repo = Repository::discover(path)?;
        
        let result = Platform::fetch_repo(&repo, pconf, options.clone(), usettings, animation)?;
        
        let mut logs = result.logs;
        match result.id {
            Some(commit) => {
                let result = Platform::merge_fetch(&repo, &result.branch, commit, options.force, animation)?;
                logs.push(result.0);
                Ok((logs, result.1))
            },
            None => Ok((logs, true)),
        }
    }
}
