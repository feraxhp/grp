use git2::{Error, Repository};
use crate::girep::local::git_utils::structure::GitUtils;

impl GitUtils {
    pub(crate) fn get_branch_name(repo: &Repository) -> Result<String, Error> {
        let head = repo.head()?;
        Ok(head.shorthand().unwrap_or("").to_string())
    }
}