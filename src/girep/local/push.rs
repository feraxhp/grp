use std::path::PathBuf;
use git2::{Repository, PushOptions, Cred, RemoteCallbacks, Direction, BranchType, Reference, Remote};
use crate::errors::error::Error;
use crate::girep::config::Config;
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;

impl Platform {
    pub(crate) fn push_repo(
        path: PathBuf,
        remote: &str,
        branch: Option<&str>,
        force: bool,
        tags: bool,
        delete: bool,
        mirror: bool,
        conf: Config
    ) -> Result<String, Error> {

        let error_mapper = |e| { Error::git_to_local(e, path.to_str().unwrap().to_string(), conf.pconf.clone()) };

        let repo = Repository::open(path.clone()).map_err(error_mapper)?;

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(GitUtils::get_credential_callbacks(conf.clone()));

        let branch_name = match branch {
            Some(name) => name.to_string(),
            None => {
                let head = repo.head().map_err(error_mapper)?;
                head.shorthand().unwrap_or("main").to_string()
            }
        };

        // Añade flags según los parámetros
        let force = if force { "+" } else { "" };
        let mut refspecs2push: Vec<&str> = vec![];

        let refs = if delete { format!(":{}", branch_name) }
        else if mirror { "refs/*:refs/*".to_string() }
        else if tags { "refs/tags/*:refs/tags/*".to_string() }
        else { format!("{force}refs/heads/{}:{force}refs/heads/{}", branch_name, branch_name, force = force) };

        let refs = refs.as_str();
        refspecs2push.push(refs);

        let mut remote = repo.find_remote(remote).map_err(error_mapper)?;

        remote.push(&refspecs2push, Some(&mut push_options)).map_err(error_mapper)?;

        Ok("".to_string())
    }
}
