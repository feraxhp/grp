use crate::errors::error::Error;
use crate::girep::config::Config;
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;
use color_print::cformat;
use git2::{BranchType, PushOptions, Repository};
use std::cmp::PartialEq;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
        path: PathBuf,
        options: Options,
        conf: Config
    ) -> Result<Vec<String>, Error> {
        let error_mapper = Error::git_to_local_mapper(path.clone(), conf.clone());
        let error_mapper = error_mapper.as_ref();

        let repo = Repository::open(path.clone()).map_err(error_mapper)?;

        let head = repo.head().map_err(error_mapper)?;

        let branch_name = match options.branch {
            Some(name) => name,
            None => head.shorthand().unwrap_or("").to_string()
        };

        if branch_name.clone() == "" { return Err(
            Error::new_custom(
                "No branch found".to_string(),
                vec![
                    cformat!("* <m>Is this a git repository?</>"),
                ]
            )
        )};

        let branch = repo
            .find_branch(branch_name.as_str(), BranchType::Local)
            .map_err(error_mapper)?;

        if options.method.clone() == Methods::UPSTREAM && !options.dry_run.clone() {
            let mut conf = repo.config().map_err(error_mapper)?;

            let merge_ref = format!("refs/heads/{}", branch_name.clone());

            conf.set_str(
                &format!("branch.{}.remote", branch_name.clone()),
                options.remote.clone().unwrap().as_str()
            ).map_err(error_mapper)?;

            conf.set_str(
                &format!("branch.{}.merge",branch_name.clone()),
                &merge_ref
            ).map_err(error_mapper)?;
        };

        let remote_name: String = match options.remote.clone() {
            None => {
                let upstream = match branch.upstream(){
                    Ok(s) => s,
                    Err(e) => {
                        println!("{}", e.message());
                        let remotes = repo.remotes().map_err(error_mapper)?;
                        return if remotes.len() == 0 {
                            Err(
                                error_mapper(
                                    git2::Error::new(
                                        git2::ErrorCode::NotFound,
                                        git2::ErrorClass::Config,
                                        ""
                                    )
                                )
                            )
                        } else {
                            let remote = remotes.get(0).unwrap_or("[remote]");
                            Err(
                                Error::new_custom(
                                    "No upstream set".to_string(),
                                    vec![
                                        cformat!("* The current branch has no Upstream set"),
                                        cformat!("  You can do it by running the command:"),
                                        cformat!("  •<g> grp push -u {} {}</>", remote, branch_name),
                                    ]
                                )
                            )
                        }
                    }
                };

                let remote = upstream.get()
                    .name()
                    .ok_or_else(
                        || Error::new_custom(
                        "Invalid upstream reference".to_string(),
                        vec![
                            cformat!("<r>* Something went wrong while getting the remote</>"),
                            cformat!("  Try by adding the remote by hand!"),
                        ]
                    )
                )?;

                let remote_name = repo.branch_remote_name(remote).map_err(error_mapper)?;
                let remote_name = remote_name
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| Error::new_custom(
                        "Invalid upstream reference".to_string(),
                        vec![
                            cformat!("<r>* Something went wrong while getting the remote</>"),
                            cformat!("  Try by adding the remote by hand!"),
                        ]
                    )
                )?;

                Ok(remote_name.to_string())
            }
            Some(name) => { Ok(name) }
        }?;

        let force = if options.force.clone() { "+" } else { "" };
        let mut ref_specs2push: Vec<&str> = vec![];

        let refs = options.method.get_refs(branch_name.as_str(), force);
        let refs = refs.as_str();

        ref_specs2push.push(refs);

        let mut remote = repo.find_remote(remote_name.as_str()).map_err(error_mapper)?;

        let messages = Arc::new(Mutex::new(Vec::<String>::new()));
        if !options.dry_run {
            let mut callbacks = GitUtils::get_credential_callbacks(conf.clone());

            let mut push_options = PushOptions::new();
            let transfer = Arc::new(Mutex::new(0));
            let forced = Arc::new(Mutex::new(options.force));

            let transfer2update = Arc::clone(&transfer);
            let messages2update = Arc::clone(&messages);
            callbacks.push_update_reference(move |refs, status| {
                let mut messages = messages2update.lock().unwrap();
                if let Some(error) = status {
                    return Err(
                        git2::Error::from_str(
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
            callbacks.push_transfer_progress(move |_current, total, _bytes| {
                let mut transfer_value = transfer2progress.lock().unwrap();
                *transfer_value = total;
            });

            push_options.remote_callbacks(callbacks);

            remote.push(&ref_specs2push, Some(&mut push_options)).map_err(error_mapper)?;
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

