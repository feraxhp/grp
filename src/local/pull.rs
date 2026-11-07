use std::path::PathBuf;

use grp_core::animation::Animation;

use super::structs::Local;
use crate::local::git::options::Options;
use crate::animations::animation::{Subprogress, Suspend};
use crate::local::git::utils::GitUtils;
use crate::local::structs::LocalError;
use crate::usettings::structs::{Pconf, Usettings};


pub enum PullAction {
    Merge,
    Rebase
}


#[allow(dead_code)]
impl Local {
    pub(crate) fn pull_repo<A: Animation + Subprogress + Suspend + ?Sized>(
        path: &PathBuf, 
        options: Options,
        pconf: Option<Pconf>, 
        action: PullAction,
        usettings: &Usettings, 
        animation: &mut Box<A>
    ) -> Result<(Vec<String>, bool), LocalError> {
        animation.change_message("Getting the local repository ...");
        let repo = GitUtils::discover(path)?;
        
        let result = Local::fetch(&repo, pconf, options.clone(), usettings, animation)?;
        
        let rebase = match action {
            PullAction::Merge => false,
            PullAction::Rebase => true,
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
