use color_print::cformat;
use git2::{Branch, BranchType, Error, Repository};

pub(crate) fn get_remote(repo: &Repository, branch_name: &str) -> Result<String, Error> {
    let branch = repo.find_branch(branch_name, BranchType::Local)?;

    get_remote_from_branch(repo, &branch)
}

pub(crate) fn get_remote_from_branch(repo: &Repository, branch: &Branch) -> Result<String, Error> {
    let upstream = match branch.upstream(){
        Ok(s) => s,
        Err(e) => {
            println!("{}", e.message());
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
                        cformat!("  •<g> grp push -u {} {}</>", remote, branch_),
                    )
                )
            }
        }
    };

    let remote = upstream.get().name()
        .ok_or_else(
            || Error::new(
                git2::ErrorCode::NotFound,
                git2::ErrorClass::Config,
                "Error getting remote name"
            )
        )?;

    let remote_name = repo.branch_remote_name(remote)?;
    let remote_name = remote_name.as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::new(
            git2::ErrorCode::NotFound,
            git2::ErrorClass::Config,
            "Invalid upstream reference"
        ))?;

    Ok(remote_name.to_string())
}