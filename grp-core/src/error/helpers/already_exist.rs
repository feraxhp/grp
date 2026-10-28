use mynox::Formater;
use mynox::Error;
use mynox::error;

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
