use crate::config::structure::Usettings;
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;
use color_print::cformat;
use git2::{BranchType, Error, ErrorClass, ErrorCode, PushOptions, Repository};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::girep::local::git_utils::options::{Methods, Options};

impl Platform {
    pub(crate) fn push_repo(&self,
        path: &PathBuf,
        options: Options,
        usettings: &Usettings
    ) -> Result<Vec<String>, Error> {
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
        let mut ref_specs2push= options.method
            .get_ref_specs(&branch_name, options.force.clone(), None);

        let conf = usettings.get_pconf_from_remote(remote_name.as_str()).to_conf();

        let messages = Arc::new(Mutex::new(Vec::<String>::new()));

        if !options.dry_run {
            let mut callbacks = GitUtils::get_credential_callbacks(conf);

            let mut push_options = PushOptions::new();
            let transfer = Arc::new(Mutex::new(0));
            let forced = Arc::new(Mutex::new(options.force));

            let transfer2update = Arc::clone(&transfer);
            let messages2update = Arc::clone(&messages);
            callbacks.push_update_reference(move |refs, status| {
                let mut messages = messages2update.lock().unwrap();
                if let Some(error) = status {
                    return Err(
                        Error::from_str(
                            format!("Error pushing reference {}: {}", refs, error)
                                .as_str()
                        )
                    );
                }

                if *transfer2update.lock().unwrap() == 0 && !*forced.lock().unwrap() {
                    messages.push(cformat!("<g>Up-to-date! <y>» {}</>", refs));
                } else {
                    let forced = *forced.lock().unwrap();
                    if forced {
                        messages.push(cformat!("<g>→ <r>Force</r> pushed: <r>+<y>{}</>", refs))
                    } else {
                        messages.push(cformat!("<g>→ Pushed: <y>{}</>", refs))
                    }
                };

                Ok(())
            });

            let transfer2progress = Arc::clone(&transfer);
            callbacks.push_transfer_progress(move |_, total, _| {
                let mut transfer_value = transfer2progress.lock().unwrap();
                *transfer_value = total;
            });

            push_options.remote_callbacks(callbacks);

            remote.push(&ref_specs2push, Some(&mut push_options))?;
            let mut finish = messages.lock().unwrap().clone();
            finish.push(cformat!("Successfully Pushed!"));
            Ok(finish)
        } else {
            let mut step_count = 1;
            let mut finish = messages.lock().unwrap().clone();
            if options.method == Methods::UPSTREAM {
                finish.push(cformat!("{}. Set upstream:", step_count));
                finish.push(cformat!("<y>  ⁕ {}</> to branch: <y>{}</>", &remote_name, branch_name));
                step_count += 1;
            }

            finish.push(cformat!("{}. Push refs:", step_count));
            for reff in ref_specs2push {
                finish.push(cformat!("<y>  » {}</> <m>→ {}</>", reff, &remote_name));
            }
            finish.push(cformat!("Process run without errors!"));
            Ok(finish)
        }
    }
}


