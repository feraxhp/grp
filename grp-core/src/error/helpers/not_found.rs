use mynox::Formater;
use mynox::Error;
use mynox::error;

use crate::ErrorType;
use crate::structs::RequestType;


pub fn not_found(conf_name: &str, request_type: &RequestType) -> Error {
    match &request_type {
        RequestType::RepositoryDetails { owner, repo } => self::repo(conf_name, owner, repo),
        RequestType::ListRepo { owner: _ } => resource(conf_name, request_type),
        RequestType::CreateRepo { owner:_, repo:_ } => resource(conf_name, request_type),
        RequestType::DeleteRepo { permanent: _, owner, repo } => self::repo(conf_name, owner, repo),
        RequestType::GetLoggedUser => resource(conf_name, request_type),
        RequestType::ListOrg { way: _ } => resource(conf_name, request_type),
        RequestType::CreateOrg { name: _ } => resource(conf_name, request_type),
        RequestType::DeleteOrg { permanent: _, name: _ } => resource(conf_name, request_type),
        RequestType::ListIssues => resource(conf_name, request_type),
        RequestType::IssueDetails { owner, repo, number } => issue(conf_name, owner, repo, number),
        RequestType::ListIssuesComments { owner, repo, number } => issue(conf_name, owner, repo, number),
        RequestType::ListTodos => resource(conf_name, request_type),
    }
}

pub fn resource(conf_name: &str, request_type: &RequestType) -> Error {
    error!(
        ErrorType::not_found::resource,
        "The resource does not exist",
        "Please check if the resource was deleted",
        "or if you does not have access to it",
        => request_type.as_tip()
        ,  format!("» **pconf**: _{}_", conf_name)
    )
}

pub fn issue(conf_name: &str, owner: &str, repo: &str, number: &u64) -> Error {
    error!(
        ErrorType::not_found::issue,
        "The issue does not exist",
        "Please check the issue and repo path",
        => format!("`{}:{}/{}#{}`", conf_name, owner, repo, number).as_tip()
    )
}

pub fn repo(conf_name: &str, owner: &str, repo: &str) -> Error {
    error!(
        ErrorType::not_found::repository,
        "The repository does not exist",
        "Please check the repository path",
        => format!("`{}:{}/{}`", conf_name, owner, repo).as_tip()
    )
}

pub fn user(conf_name: &str, owner: &str) -> Error {
    error!(
        ErrorType::not_found::user,
        "The user does not exist",
        "Please check if the name is correct",
        "or if is the correct repo"
        => format!("» **owner**: _{}_", owner)
        ,  format!("» **pconf**: _{}_", conf_name)
    )
}
