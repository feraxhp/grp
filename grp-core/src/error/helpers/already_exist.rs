use mynox::Formater;
use mynox::Error;
use mynox::error;

use crate::structs::PullRequestOptions;
use crate::ErrorType;


pub fn repo(conf_name: &str, owner: &str, repo: &str) -> Error {
    error!(
        ErrorType::existing::repository,
        "The repository already exist",
        "Repositoriy combination should be unique",
        => format!("`{}:-{}/{}-`", conf_name, owner, repo).as_tip()
    )
}

pub fn organization(name: &str) -> Error {
    error!(
        ErrorType::existing::organization,
        "The organization already exist",
        "Organization name should be unique",
        => format!("» **name**: _{}_", name)
    )
}

pub fn pull_request(conf_name: &str, options: &PullRequestOptions) -> Error {
    error!(
        ErrorType::existing::pull_request,
        "The pull request already exist",
        "Pull request between these branches already exist",
        => format!("`{}:{}/{}?{}` -> `{}:{}/{}?{}`",
            conf_name, options.origin.owner, options.origin.repo, options.origin.branch,
            conf_name, options.target.owner, options.target.repo, options.target.branch,
        ).as_tip()
    )
}
