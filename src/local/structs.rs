
use std::{fmt::Display, ops::Deref};
use grp_core::{Config, Error, Platform};

use crate::usettings::structs::Usettings;

use super::git::structs::Action;

pub struct Local(pub Platform);

impl Deref for Local {
    type Target = Platform;
    fn deref(&self) -> &Self::Target { &self.0 }
}

pub trait Git2Error {
    fn from_git2<
        T: Display,
        S: Display,
    >(
        ge: git2::Error, 
        action: Action, 
        owner: S,
        repo: T, 
        config: Option<&Config>,
        usettings: &Usettings,
    ) -> Error;
}