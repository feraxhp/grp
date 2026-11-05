use std::fmt::Display;

use grp_core::animation::Animation;
use mynox::{Error, Formater, error};
use git2::Repository;
use reqwest::Url;

use crate::commands::validations::or_exit::structure::OrExit;
use crate::commands::validations::branch::BranchStructure;
use crate::commands::validations::repo::RepoStructure;
use crate::usettings::structs::Usettings;
use crate::local::git::structs::GitUtils;
use crate::local::structs::LocalError;


pub fn detect_current_repo<A: Animation + ?Sized>(usettings: &Usettings, animation: &Box<A>, ask_default: bool) -> Result<(BranchStructure, String, Repository), LocalError> {
    let repo = git2::Repository::discover(".")?;
    let branch_name = GitUtils::get_branch_name(&repo)?;
    
    if branch_name.is_empty() {
        return Err(LocalError::Git(git2::Error::new(
            git2::ErrorCode::UnbornBranch, git2::ErrorClass::Callback,
            "Impossible to auto-detect branch",
        )));
    }
    
    let branch = repo.find_branch(&branch_name, git2::BranchType::Local)?;
    let remote = GitUtils::get_remote_from_branch(&repo, &branch)?;
    
    let pconf = match usettings.get_raw_pconf_by_name(&remote) {
        Some(p) => p,
        None => &usettings.get_raw_default_pconf().or_exit(animation),
    };
    let pconf_name = pconf.name.clone();
    
    let mut remote = repo.find_remote(&remote)?;
    let url = remote.url()?;
    
    let url = match Url::parse(url) {
        Ok(u) => u,
        Err(e) => {
            return Err(
                LocalError::Core(
                    error_autodetect(url, pconf_name)
                        .extend_detail(e.to_string())
                )
            )
        },
    };

    let full_path = match url.path_segments() {
        Some(ps) => ps.collect::<Vec<_>>(),
        None => return Err(
            LocalError::Core(
                error_autodetect(url, pconf_name)
                    .extend_note("The url should have a path to be recongnized".as_tip())
            )
        ),
    };
    
    if full_path.len() < 2 { // the path should at least be 2 deep
        return Err(LocalError::Core(error_autodetect(url, pconf_name)));
    }
    
    let last = full_path.last().unwrap();
    let path = last.replace(".git", "");
    
    let owner = full_path.get(..(full_path.len() -1)).unwrap().join("/");

    let mut default = String::new();
    if ask_default {
        remote.connect(git2::Direction::Fetch)?;
        
        let buf = remote.default_branch()?;
        default = buf.as_str()?.to_string();
    }
    
    drop(branch); drop(remote);
    
    Ok((BranchStructure { 
        repo: RepoStructure {
            pconf: Some(pconf_name),
            owner, path,
            len: full_path.len() - 1,
        }, 
        branch: branch_name
    }, default, repo))
}

fn error_autodetect(url: impl Display, remote: impl Display) -> Error {
    error!(
        "auto::detect::current::repo",
        "Imposible to autodetect the current repo structure",
        "The url of the remote is mal formed",
        => format!("» **url**: _<raw>{}</raw>_", url),
           format!("» **remote**: _<raw>{}</raw>_", remote)
    )
}
