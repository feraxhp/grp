use crate::config::structure::Usettings;
use crate::girep::local::git_utils::options::{Methods, Options};
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;
use color_print::cformat;
use git2::{AutotagOption, BranchType, Error, FetchOptions, FetchPrune};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
            fetch_options.download_tags(AutotagOption::All);

            remote.fetch(&ref_specs, Some(&mut fetch_options), None)?;
            let mut finish = messages.lock().unwrap().clone();

            let fetch_head = repo.find_reference("FETCH_HEAD")?;
            let annotated_commit = repo.reference_to_annotated_commit(&fetch_head)?;
            let commit_hash = annotated_commit.id();
            finish.push(format!("FETCH_HEAD: {}", commit_hash));

            // if do_merge {
            //     let messages = GitUtils::do_merge(&repo, &branch_name, annotated_commit, options.force.clone())?;
            //
            //     Ok((finish, true))
            // } else {
            finish.push(cformat!("Successfully fetch!"));
            Ok((finish, true))
            // }
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
}

