use std::collections::HashMap;

use git2::{BranchType, Error, ErrorClass, ErrorCode, Oid, Repository};
use crate::local::git::options::Options;

use super::super::structs::GitUtils;

impl GitUtils {
    pub(crate) fn get_branch_name(repo: &Repository) -> Result<String, Error> {
        let head = repo.head()?;
        Ok(head.shorthand().unwrap_or("").to_string())
    }
    pub(crate) fn get_repo_branch_and_remote(repo: &Repository, options: &Options) -> Result<(String, String), Error> {
        let branch_name = match &options.branch {
            Some(name) => name.clone(),
            None => GitUtils::get_branch_name(&repo)?,
        };

        if branch_name.is_empty() {
            return Err(Error::new(
                ErrorCode::UnbornBranch, ErrorClass::Callback,
                "Impossible to auto-detect branch",
            ));
        }

        let branch = repo.find_branch(&branch_name, BranchType::Local)?;

        let remote_name = match &options.remote {
            None => GitUtils::get_remote_from_branch(&repo, &branch)?,
            Some(name) => name.clone(),
        };

        drop(branch);
        Ok((branch_name, remote_name))
    }

    pub fn list_branches(repo: &Repository) -> Result<Vec<String>, Error> {
        let mut names = Vec::new();
        let branches = repo.branches(Some(BranchType::Local))?;
        
        for branch in branches {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                names.push(name.to_string());
            }
        }
        
        Ok(names)
    }
    pub(crate) fn get_branches_by_remote(repo: &Repository, remote_name: &str) -> Result<HashMap<String, Oid>, Error> {
        let mut branches = HashMap::new();
        let remote_prefix = format!("refs/remotes/{}/", remote_name);
        
        for reference in repo.references()?.filter_map(Result::ok) {
            let Some(name) = reference.name() else { continue };
            let Some(branch_name) = name.strip_prefix(&remote_prefix) else { continue };
            let Some(target) = reference.target() else { continue };
            
            branches.insert(branch_name.to_string(), target);
        }
        
        Ok(branches)
    }
}