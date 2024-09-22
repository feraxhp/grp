// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::structure::Pconf;

pub(crate) struct Config {
    pub(crate) pconf: String,
    pub(crate) user: String,
    pub(crate) token: String,
    pub(crate) endpoint: String,
}

impl Config {
    pub(crate) fn new(
        pconf: String,
        user: String,
        token: String,
        endpoint: String
    ) -> Self {
        Config {
            pconf,
            user,
            token,
            endpoint
        }
    }

    pub(crate) fn clone(&self) -> Config {
        Config::new(
            self.pconf.clone(),
            self.user.clone(),
            self.token.clone(),
            self.endpoint.clone()
        )
    }
}

impl Pconf {
    pub(crate) fn to_conf(&self) -> Config {
        Config {
            pconf: self.name.clone(),
            user: self.owner.clone(),
            token: self.token.clone(),
            endpoint: self.endpoint.clone(),
        }
    }
}