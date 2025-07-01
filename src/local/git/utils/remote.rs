use git2::{Branch, Error, Repository};

use super::super::structs::GitUtils;

impl GitUtils {
    pub(crate) fn get_remote_from_branch(repo: &Repository, branch: &Branch) -> Result<String, Error> {
        let upstream = match branch.upstream() {
            Ok(s) => s,
            Err(_) => {
                let remotes = repo.remotes()?;
                return if remotes.len() == 0 {
                    Err(
                        Error::new(
                            git2::ErrorCode::NotFound,
                            git2::ErrorClass::Config,
                            "The repository has no remotes configured",
                        )
                    )
                } else {
                    let remote = remotes.get(0).unwrap_or("[remote]");
                    let branch_ = branch.name()?.unwrap_or("[unknown]");
                    Err(
                        Error::new(
                            git2::ErrorCode::NotFound,
                            git2::ErrorClass::Config,
                            format!("-u {} {}", remote, branch_),
                        )
                    )
                }
            }
        };

        let remote = upstream.get().name().ok_or_else(
            || Error::new(
                git2::ErrorCode::NotFound,
                git2::ErrorClass::Config,
                "The repository has a remote configured\nwith a inaccesible name.",
            )
        )?;

        let remote_name = repo.branch_remote_name(remote)?;
        let remote_name = remote_name.as_str().map(|s| s.to_owned()).ok_or_else(
            || Error::new(
                git2::ErrorCode::NotFound,
                git2::ErrorClass::Config,
                "The repository has a remote configured\nwith an inaccesible name.",
            )
        )?;

        Ok(remote_name.to_string())
    }
}