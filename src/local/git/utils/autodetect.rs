use std::fmt::Display;

use mynox::Formater;
use mynox::error;
use reqwest::Url;

use crate::usettings::structs::RawPconf;
use crate::local::structs::LocalError;

use super::GitUtils;


pub struct RepoInformation {
    pub depth: usize,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub remote: String,
}


impl GitUtils {
    pub fn autodetect(&self, pconf: Option<RawPconf>) -> Result<RepoInformation, LocalError> {
        let branch_name = self.get_branch_name()?;
        
        if branch_name.is_empty() {
            return Err(LocalError::Git(git2::Error::new(
                git2::ErrorCode::UnbornBranch, git2::ErrorClass::Callback,
                "Impossible to auto-detect branch",
            )));
        }
        
        if &branch_name == "HEAD" {
            return Err(LocalError::Core(
                error!(
                    "grp::local::branch::detached", "Unkown branch",
                    "You have a detach *HEAD*, that makes",
                    "imposible to autodetect the _branch_"
                )
            ));
        }
        
        let branch = self.find_branch(&branch_name, git2::BranchType::Local)?;
        
        let remote_name = match pconf {
            Some(s) => s.name,
            None => self.get_remote_from_branch(&branch)?,
        };
        
        let remote = self.find_remote(&remote_name)?;
        let url = remote.url()?;
        
        let url = match Url::parse(url) {
            Ok(u) => u,
            Err(e) => {
                return Err(
                    LocalError::Core(
                        error_autodetect(url, remote_name)
                            .extend_detail(e.as_tip())
                    )
                )
            },
        };
        
        let full_path = match url.path_segments() {
            Some(ps) => ps.collect::<Vec<_>>(),
            None => return Err(
                LocalError::Core(
                    error_autodetect(url, remote_name)
                        .extend_note("The url should have a path to be recongnized".as_tip())
                )
            ),
        };
        
        if full_path.len() < 2 { // the path should at least be 2 deep
            return Err(LocalError::Core(error_autodetect(url, remote_name)));
        }
        
        let last = full_path.last().unwrap();
        let path = last.strip_suffix(".git")
            .unwrap_or(last)
            .to_string()
        ;
        
        let owner = full_path.get(..(full_path.len() -1)).unwrap().join("/");
        if owner.is_empty() {
            return Err(LocalError::Core(error_autodetect(url, remote_name)));
        }
        
        Ok(
            RepoInformation { 
                depth: (full_path.len() - 1), 
                owner, 
                repo: path, 
                branch: branch_name, 
                remote: remote_name, 
            }
        )
    }
}

fn error_autodetect(url: impl Display, remote: impl Display) -> mynox::Error {
    error!(
        "auto::detect::current::repo",
        "Imposible to autodetect the current repo structure",
        "The url of the remote is mal formed",
        => format!("» **url**: _<raw>{}</raw>_", url),
           format!("» **remote**: _<raw>{}</raw>_", remote)
    )
}
