use std::collections::HashMap;

use git2::{BranchType, Error, ErrorClass, ErrorCode, Oid};
use crate::local::git::options::Options;

use super::super::structs::GitUtils;

impl GitUtils {
    pub(crate) fn get_branch_name(&self) -> Result<String, Error> {
        let head = self.head()?;
        Ok(head.shorthand().unwrap_or("").to_string())
    }
    pub(crate) fn get_repo_branch_and_remote(&self, options: &Options) -> Result<(String, String), Error> {
        let branch_name = match &options.branch {
            Some(name) => name.clone(),
            None => self.get_branch_name()?,
        };

        if branch_name.is_empty() {
            return Err(Error::new(
                ErrorCode::UnbornBranch, ErrorClass::Callback,
                "Impossible to auto-detect branch",
            ));
        }

        let branch = self.find_branch(&branch_name, BranchType::Local)?;

        let remote_name = match &options.remote {
            None => self.get_remote_from_branch(&branch)?,
            Some(name) => name.clone(),
        };

        drop(branch);
        Ok((branch_name, remote_name))
    }

    pub fn list_branches(&self) -> Result<Vec<String>, Error> {
        let mut names = Vec::new();
        let branches = self.branches(Some(BranchType::Local))?;
        
        for branch in branches {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                names.push(name.to_string());
            }
        }
        
        Ok(names)
    }
    pub(crate) fn get_branches_by_remote(&self, remote_name: &str) -> Result<HashMap<String, Oid>, Error> {
        let mut branches = HashMap::new();
        let remote_prefix = format!("refs/remotes/{}/", remote_name);
        
        for reference in self.references()?.filter_map(Result::ok) {
            let Ok(name) = reference.name() else { continue };
            let Some(branch_name) = name.strip_prefix(&remote_prefix) else { continue };
            let Some(target) = reference.target() else { continue };
            
            branches.insert(branch_name.to_string(), target);
        }
        
        Ok(branches)
    }
}