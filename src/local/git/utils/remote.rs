use git2::{Branch, Error, ErrorClass, ErrorCode};
use grp_core::Config;

use super::super::structs::GitUtils;

impl GitUtils {
    pub(crate) fn get_remote_from_branch(&self, branch: &Branch) -> Result<String, Error> {
        let branch_ref_name = branch.get().name()?;
        
        let remote_buf = match self.branch_upstream_remote(branch_ref_name) {
            Ok(buf) => Ok(buf),
            Err(error) if ( matches!(
                (error.code(), error.class()), 
                (ErrorCode::NotFound, ErrorClass::Config)
            ) ) => {
                let remotes = self.remotes()?;
                if remotes.is_empty() {
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
    
    pub(crate) fn get_default_branch(&self, remote_name: &str, config: &Config) -> Result<String, Error> {
        let mut remote = self.find_remote(remote_name)?;
        
        let cb = GitUtils::get_credential_callbacks(config);
        remote.connect_auth(git2::Direction::Fetch, Some(cb), None)?;
        
        let buf = remote.default_branch()?;
        let buf = buf.as_str()?;
        
        Ok(
            buf
                .strip_prefix("refs/heads/")
                .unwrap_or(buf)
                .to_string()
        )
    }
}