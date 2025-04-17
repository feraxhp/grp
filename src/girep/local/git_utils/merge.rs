use color_print::cformat;
use git2::{AnnotatedCommit, Repository, Error};
use crate::girep::local::git_utils::structure::GitUtils;
use crate::update::metadata::Version;

impl GitUtils {
    fn merge(
        repo: &Repository,
        local: &AnnotatedCommit,
        remote: &AnnotatedCommit,
    ) -> Result<String, Error> {
        let local_tree = repo.find_commit(local.id())?.tree()?;
        let remote_tree = repo.find_commit(remote.id())?.tree()?;

        let merge_base = repo.merge_base(local.id(), remote.id())?;
        let ancestor = repo.find_commit(merge_base)?.tree()?;
        let mut merge_state = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

        if merge_state.has_conflicts() {
            repo.checkout_index(Some(&mut merge_state), None)?;
            return Ok(cformat!("Merge conflicts detected..."));
        }

        let result_tree = repo.find_tree(merge_state.write_tree_to(repo)?)?;

        let msg = format!("Merge: {} into {}", remote.refname().unwrap_or("remote"), local.refname().unwrap_or("local"));

        let sig = repo.signature()?;
        let local_commit = repo.find_commit(local.id())?;
        let remote_commit = repo.find_commit(remote.id())?;


        let merge_commit = repo.commit(
            Some("HEAD"), &sig, &sig, &msg, &result_tree,
            &[&local_commit, &remote_commit],
        )?;

        repo.checkout_head(None)?;

        Ok(cformat!("<m, i>Merge:</> <y>{}</> <g>into</> <y>{}</>", remote.refname().unwrap_or("remote"), local.refname().unwrap_or("local")))
    }
}