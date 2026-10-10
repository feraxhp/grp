
use std::ops::Deref;
use grp_core::{Config, Error, Platform};

use crate::usettings::structs::Usettings;

use super::git::structs::Action;

pub struct Local(pub Platform);

impl Deref for Local {
    type Target = Platform;
    fn deref(&self) -> &Self::Target { &self.0 }
}

pub struct Git2Context<'a> {
    pub action: Action, 
    pub owner: &'a str,
    pub repo: &'a str, 
    pub config: Option<&'a Config>,
    pub usettings: &'a Usettings,
}

pub enum LocalError {
    Core(Error),
    Git(git2::Error),
}

pub trait ToError {
    type Context<'a>;
    
    fn to_error<'a>(&self, context: Self::Context<'a>) -> Error;
}

impl From<git2::Error> for LocalError {
    fn from(err: git2::Error) -> Self {
        LocalError::Git(err)
    }
}

impl From<Error> for LocalError {
    fn from(err: Error) -> Self {
        LocalError::Core(err)
    }
}