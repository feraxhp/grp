use std::collections::HashMap;
use git2::{Error, Oid, Repository};
use crate::girep::local::git_utils::structure::GitUtils;

impl GitUtils {
    pub(crate) fn get_branch_name(repo: &Repository) -> Result<String, Error> {
        let head = repo.head()?;
        Ok(head.shorthand().unwrap_or("").to_string())
    }
    pub(crate) fn get_branches_by_remote(repo: &Repository, remote_name: &str) -> Result<HashMap<String, Oid>, Error> {
        let mut branches = HashMap::new();
        let remote_prefix = format!("refs/remotes/{}/", remote_name);

        for reference in repo.references()? {
            if let Ok(reference) = reference {
                if let Some(name) = reference.name() {
                    if name.starts_with(&remote_prefix) {

                        if let Some(branch_name) = name.strip_prefix(&remote_prefix) {
                            if let Some(target) = reference.target() {
                                branches.insert(branch_name.to_string(), target);
                            }
                        }
                    }
                }
            }
        }

        Ok(branches)
    }
}