use std::path::PathBuf;

use git2::{Error, Repository};
use grp_core::animation::Animation;

use super::structs::Local;
use crate::local::git::options::Options;
use crate::animations::animation::Subprogress;
use crate::usettings::structs::{Pconf, Usettings};


pub enum PullAction {
    MERGE,
    REBASE
}


#[allow(dead_code)]
impl Local {
    pub(crate) fn pull_repo<A: Animation + Subprogress + ?Sized>(
        path: &PathBuf, 
        options: Options,
        pconf: Option<Pconf>, 
        action: PullAction,
        usettings: &Usettings, 
        animation: &mut Box<A>
    ) -> Result<(Vec<String>, bool), Error> {
        animation.change_message("Getting the local repository ...");
        let repo = Repository::discover(path)?;
        
        let result = Local::fetch(&repo, pconf, options.clone(), usettings, animation)?;
        
        let rebase = match action {
            PullAction::MERGE => false,
            PullAction::REBASE => true,
        };
        
        let mut logs = result.logs;
        match result.id {
            Some(commit) => {
                let result = Local::merge_fetch(&repo, &result.branch, commit, options.force, rebase, animation)?;
                logs.push(result.0);
                Ok((logs, result.1))
            },
            None => Ok((logs, true)),
        }
    }
}
