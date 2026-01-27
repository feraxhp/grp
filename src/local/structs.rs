
use std::ops::Deref;
use grp_core::{Config, Error, Platform};

use super::git::structs::Action;

pub struct Local(pub Platform);

impl Deref for Local {
    type Target = Platform;
    fn deref(&self) -> &Self::Target { &self.0 }
}

pub trait Git2Error {
    fn from_git2<T: AsRef<str>>(ge: git2::Error, action: Action, repo: &T, config: Option<&Config>) -> Error;
}