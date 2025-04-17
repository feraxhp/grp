use std::path::PathBuf;
use std::rc::Rc;
use git2::{Branch, BranchType, Error, ErrorClass, ErrorCode, Repository};
use crate::girep::local::git_utils::options::Options;
use crate::girep::local::git_utils::structure::GitUtils;

impl GitUtils {
    pub(crate) fn get_repo_branch_and_remote(path: &PathBuf, options: &Options) -> Result<(Repository, String, String), Error> {
        let repo = Repository::discover(path.clone())?;

        let branch_name = match &options.branch {
            Some(name) => name.clone(),
            None => GitUtils::get_branch_name(&repo)?,
        };

        if branch_name.is_empty() {
            return Err(Error::new(
                ErrorCode::UnbornBranch,
                ErrorClass::Callback,
                "Impossible to auto-detect branch",
            ));
        }

        let branch = repo.find_branch(&branch_name, BranchType::Local)?;

        let remote_name = match &options.remote {
            None => GitUtils::get_remote_from_branch(&repo, &branch)?,
            Some(name) => name.clone(),
        };

        drop(branch);
        Ok((repo, branch_name, remote_name))
    }
}