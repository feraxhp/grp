use std::collections::HashSet;
use crate::girep::config::structure::Usettings;
use crate::girep::local::git_utils::options::{Methods, Options};
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;
use color_print::cformat;
use git2::{AutotagOption, BranchType, Error, ErrorClass, ErrorCode, FetchOptions, FetchPrune, Repository};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use git2::build::CheckoutBuilder;

impl Platform {
    pub(crate) fn fetch_repo<F>(&self,
        path: &PathBuf, options: Options,
        usettings: &Usettings, do_merge: bool,
        mut print: F,
    ) -> Result<(Vec<String>, bool), Error>
    where
        F: FnMut(String)
    {

        let (
            repo,
            branch_name,
            remote_name
        ) = GitUtils::get_repo_branch_and_remote(path, &options)?;
        let branch = repo.find_branch(&branch_name, BranchType::Local)?;

        if options.method == Methods::UPSTREAM && !options.dry_run.clone() {
            let _ = options.method.set_upstream(&repo, &branch_name, &remote_name)?;
        };

        let mut remote = repo.find_remote(remote_name.as_str())?;
        let mut ref_specs = options.method
            .get_ref_specs(&branch_name, options.force.clone(), Some(&remote));

        let conf = usettings.get_pconf_from_remote(remote_name.as_str()).to_conf();

        let messages = Arc::new(Mutex::new(Vec::<String>::new()));

        if !options.dry_run {
            let mut callbacks = GitUtils::get_credential_callbacks(conf);

            let mut fetch_options = FetchOptions::new();
            let transfer = Arc::new(Mutex::new(0));
            let forced = Arc::new(Mutex::new(options.force));

            let transfer2update = Arc::clone(&transfer);
            let messages2update = Arc::clone(&messages);
            callbacks.transfer_progress(|stats| {
                if stats.received_objects() == stats.total_objects() {
                    print(format!(
                        "Resolving deltas {}/{}",
                        stats.indexed_deltas(),
                        stats.total_deltas()
                    ));
                } else if stats.total_objects() > 0 {
                    print(format!(
                        "Received {}/{} objects ({}) in {} bytes",
                        stats.received_objects(),
                        stats.total_objects(),
                        stats.indexed_objects(),
                        stats.received_bytes()
                    ));
                }
                true
            });

            let transfer2progress = Arc::clone(&transfer);
            callbacks.pack_progress(move |_, total, _| {
                let mut transfer_value = transfer2progress.lock().unwrap();
                *transfer_value = total;
            });

            fetch_options.remote_callbacks(callbacks);
            fetch_options.download_tags(AutotagOption::Auto);

            let branches_before = GitUtils::get_branches_by_remote(&repo, &remote_name)?;

            remote.fetch(&ref_specs, Some(&mut fetch_options), None)?;
            let mut finish = messages.lock().unwrap().clone();

            let branches_after = GitUtils::get_branches_by_remote(&repo, &remote_name)?;
            let new_branches = branches_after.iter()
                .filter(|(name, _)| !branches_before.contains_key(name.clone()))
                .for_each(|(name, _)| {
                    finish.push(cformat!("<g>+ branch:</> <y>{}/{}</>", &remote_name, name));
                });

            let fetch_head = repo.find_reference("FETCH_HEAD")?;
            let annotated_commit = repo.reference_to_annotated_commit(&fetch_head)?;

            if do_merge {
                let message = self.merge_fetch(
                    &repo, &branch_name,
                    annotated_commit, options.force.clone()
                )?;
                finish.push(message);

                finish.push(cformat!("Successfully fetch!"));
                Ok((finish, true))
            } else {
                finish.push(cformat!("Successfully fetch!"));
                Ok((finish, true))
            }
        } else {
            let mut step_count = 1;
            let mut finish = messages.lock().unwrap().clone();

            if options.method == Methods::UPSTREAM {
                finish.push(cformat!("{}. Add remote:", step_count));
                finish.push(cformat!("<y>  ⁕ {}</> to branch: <y>{}</>", remote_name.clone(), branch_name));
                step_count += 1;
            }

            finish.push(cformat!("{}. Pull refs:", step_count));
            for reff in ref_specs {
                finish.push(cformat!("<y>  » {}</> <m>→ {}</>", remote_name.clone(), reff ));
            }
            finish.push(cformat!("Process run without errors!"));
            Ok((finish, true))
        }

    }

    fn merge_fetch<'a>(&self,
        repo: &'a Repository,
        remote_branch: &str,
        fetch_commit: git2::AnnotatedCommit<'a>,
        force: bool,
    ) -> Result<String, Error> {

        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        match analysis.0 {

            a if a.is_fast_forward() => {
                let refname = format!("refs/heads/{}", remote_branch);


                match repo.find_reference(&refname) {
                    Ok(mut r) => Ok(GitUtils::fast_forward(repo, &mut r, &fetch_commit, force)?),

                    Err(_) => {
                        repo.reference(&refname, fetch_commit.id(), true, "")?;

                        repo.set_head(&refname)?;

                        let mut builder = CheckoutBuilder::default();

                        builder.allow_conflicts(true);
                        builder.conflict_style_merge(true);
                        builder.force();

                        repo.checkout_head(Some(&mut builder))?;

                        Ok(cformat!(
                            "<m>Setting:</> <y>{}</> to <m>{}</>",
                            remote_branch, fetch_commit.id()
                        ))
                    }
                }
            },

            a if a.is_normal() => {
                let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
                Ok(GitUtils::merge(&repo, &head_commit, &fetch_commit)?)
            },

            a if a.is_unborn() => {
                let refname = format!("refs/heads/{}", remote_branch);

                repo.reference(&refname, fetch_commit.id(), true, "")?;
                repo.set_head(&refname)?;


                repo.checkout_head(Some(CheckoutBuilder::default().force(),))?;

                Ok(cformat!(
                    "<m>Initialized repository with:</> <y>{}</> at <m>{}</>",
                    remote_branch, fetch_commit.id()
                ))
            },

            a if a.is_up_to_date() => {
                Ok(cformat!("<g>up-to-date: <y>{}</>", remote_branch))
            },

            a => {
                // For debugging or logging purposes
                let analysis_description = format!(
                    "fast_forward: {}, normal: {}, up_to_date: {}, unborn: {}",
                    a.is_fast_forward(), a.is_normal(), a.is_up_to_date(), a.is_unborn()
                );

                Ok(cformat!("No action taken. Merge analysis: {}", analysis_description))
            }
        }
    }
}

