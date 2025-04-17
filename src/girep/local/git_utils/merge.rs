use crate::girep::local::git_utils::structure::GitUtils;
use color_print::cformat;
use git2::{AnnotatedCommit, Error, ErrorClass, ErrorCode, Reference, Repository, StatusOptions};
use std::io::Write;

impl GitUtils {
    pub(crate) fn merge(
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

            let their_head = repo.find_reference("FETCH_HEAD")?.target().unwrap();
            repo.reference("MERGE_HEAD", their_head, false, "merge: recording MERGE_HEAD")?;

            let merge_msg = repo.path().join("MERGE_MSG");
            let mut file = std::fs::File::create(merge_msg).unwrap();
            let message = format!("Merge branch '{}' into {}", remote.id(), local.id());
            file.write_all(message.as_bytes()).unwrap();

            return Err(Error::new(
                ErrorCode::Conflict,
                ErrorClass::Merge,
                cformat!("{},{}", remote.id(), local.id()),
            ))
        }

        let result_tree = repo.find_tree(merge_state.write_tree_to(repo)?)?;

        let from = remote.id();
        let to = local.id();

        let msg = format!("Merge: {} into {}", &from, &to);

        let sig = repo.signature()?;
        let local_commit = repo.find_commit(local.id())?;
        let remote_commit = repo.find_commit(remote.id())?;


        let merge_commit = repo.commit(
            Some("HEAD"), &sig, &sig, &msg, &result_tree,
            &[&local_commit, &remote_commit],
        )?;

        let mut checkout_builder = git2::build::CheckoutBuilder::default();
        checkout_builder.force();

        repo.checkout_head(Some(&mut checkout_builder))?;

        Ok(cformat!("<m>Merge:</> <y>{}</> <g>into</> <y>{}</>", from, to))
    }

    pub(crate) fn fast_forward(
        repo: &Repository,
        reference: &mut Reference,
        remote: &AnnotatedCommit,
        force: bool,
    ) -> Result<String, Error> {

        if !force {
            let mut status_option = StatusOptions::new();

            status_option.include_ignored(false);
            status_option.include_untracked(true);

            let statuses = repo.statuses(Some(&mut status_option))?;

            if !statuses.is_empty() {

                let mut changed_files = Vec::new();
                for entry in statuses.iter() {
                    if let Some(path) = entry.path() {
                        changed_files.push(path.to_string());
                    }
                }

                return Err(
                    Error::new(
                        ErrorCode::Locked,
                        ErrorClass::Merge,
                        format!("{}", changed_files.join(",")),
                    )
                )
            }
        }

        let name = reference.name()
            .map(ToString::to_string).unwrap_or_else(
                || String::from_utf8_lossy(reference.name_bytes()).to_string()
        );

        let msg = format!("Fast-Forward: Setting {} to id: {}", name, remote.id());

        reference.set_target(remote.id(), &msg)?;
        repo.set_head(&name)?;

        let mut checkout_builder = git2::build::CheckoutBuilder::default();
        checkout_builder.force();

        repo.checkout_head(Some(&mut checkout_builder))?;
        let to = remote.id();

        Ok(cformat!("<m>Fast-Forward:</> <y>{}</> to id: <y>{}</>", name, to))
    }
}