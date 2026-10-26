use mynox::Formater;
use mynox::Error;
use mynox::error;

use crate::ErrorType;


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
        ErrorType::not_found::repository,
        "The user does not exist",
        "Please check if the name is correct",
        "or if is the correct repo"
        => format!("» **code**: _{}_", owner)
        ,  format!("» **pconf**: _{}_", conf_name)
    )
}
