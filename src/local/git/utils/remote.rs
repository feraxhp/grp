use git2::{Branch, Error, ErrorClass, ErrorCode, Repository};

use super::super::structs::GitUtils;

impl GitUtils {
    pub(crate) fn get_remote_from_branch(repo: &Repository, branch: &Branch) -> Result<String, Error> {
        let branch_ref_name = branch.get().name()?;
        
        let remote_buf = match repo.branch_upstream_remote(branch_ref_name) {
            Ok(buf) => Ok(buf),
            Err(error) if ( matches!(
                (error.code(), error.class()), 
                (ErrorCode::NotFound, ErrorClass::Config)
            ) ) => {
                let remotes = repo.remotes()?;
                if remotes.len() == 0 {
                    Err(
                        Error::new(
                            git2::ErrorCode::NotFound,
                            git2::ErrorClass::Config,
                            "The repository has no remotes configured",
                        )
                    )
                } else {
                    let remote = remotes.get(0).unwrap_or(None).unwrap_or("[remote]");
                    let branch_ = branch.name()?.unwrap_or("[unknown]");
                    Err(
                        Error::new(
                            git2::ErrorCode::NotFound,
                            git2::ErrorClass::Config,
                            format!("-u {} {}", remote, branch_),
                        )
                    )
                }
            },
            error => error
        }?;
        
        let remote_name = remote_buf.as_str()?;
        Ok(remote_name.to_string())
    }
}