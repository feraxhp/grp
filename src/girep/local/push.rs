use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;
use color_print::cformat;
use git2::{BranchType, Error, ErrorClass, ErrorCode, PushOptions, Repository};
use std::cmp::PartialEq;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::config::structure::{Pconf, Usettings};
use crate::girep::local::git_utils::branch::get_branch_name;
use crate::girep::local::git_utils::remote::get_remote_from_branch;

#[derive(PartialEq, Clone)]
pub(crate) enum Methods {
    DEFAULT,
    ALL,
    BRANCHES,
    TAGS,
    UPSTREAM
}

pub(crate) struct Options {
    pub(crate) method: Methods,
    pub(crate) remote: Option<String>,
    pub(crate) branch: Option<String>,
    pub(crate) force: bool,
    pub(crate) dry_run: bool
}

impl Platform {
    pub(crate) fn push_repo(&self,
        path: &PathBuf,
        options: Options,
        usettings: &Usettings
    ) -> Result<Vec<String>, Error> {
        let repo = Repository::open(path.clone())?;

        let branch_name = match options.branch {
            Some(name) => name,
            None => get_branch_name(&repo)?
        };

        if branch_name == "" { return Err(
            Error::new(
                ErrorCode::UnbornBranch,
                ErrorClass::Callback,
                "Imposible to auto-detect branch"
            )
        )};

        let branch = repo.find_branch(branch_name.as_str(), BranchType::Local)?;

        let remote_name = match options.remote {
            None => { get_remote_from_branch(&repo, &branch)? }
            Some(name) => { name }
        };

        if options.method == Methods::UPSTREAM && !options.dry_run.clone() {
            let mut conf = repo.config()?;

            let merge_ref = format!("refs/heads/{}", &branch_name);

            conf.set_str(&format!("branch.{}.remote", &branch_name), &remote_name)?;
            conf.set_str(&format!("branch.{}.merge", &branch_name), &merge_ref)?;
        };

        let force = if options.force { "+" } else { "" };
        let mut ref_specs2push: Vec<&str> = vec![];

        let refs = options.method.get_refs(branch_name.as_str(), force);
        let refs = refs.as_str();
        ref_specs2push.push(refs);

        let mut remote = repo.find_remote(remote_name.as_str())?;

        let messages = Arc::new(Mutex::new(Vec::<String>::new()));

        let conf = match usettings.get_pconf(remote_name.clone()) {
            None => { usettings.get_default().to_conf() },
            Some(pconf) => { pconf.to_conf() }
        };

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
                    messages.push(
                        cformat!("<g>Pushed: <r>{}<y>{}</>", if *forced.lock().unwrap() { "+"} else {""}, refs)
                    )
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
                finish.push(cformat!("{}. Add remote:", step_count));
                finish.push(cformat!("<y>  ⁕ {}</> to branch: <y>{}</>", remote_name.clone(), branch_name));
                step_count += 1;
            }

            finish.push(cformat!("{}. Push refs:", step_count));
            for reff in ref_specs2push {
                finish.push(cformat!("<y>  » {}</> <m>→ {}</>", reff, remote_name.clone()));
            }
            finish.push(cformat!("Process run without errors!"));
            Ok(finish)
        }
    }
}

impl Methods {
    fn get_refs(&self, branch: &str, force: &str) -> String {
        match self {
            Methods::DEFAULT |
            Methods::UPSTREAM => format!("{}refs/heads/{branch}:refs/heads/{branch}", force, branch = branch),
            Methods::ALL => format!("{}refs/*:refs/*", force),
            Methods::BRANCHES => format!("{}refs/heads/*:refs/heads/*", force),
            Methods::TAGS => format!("{}refs/tags/*:refs/tags/*", force)
        }
    }
}

