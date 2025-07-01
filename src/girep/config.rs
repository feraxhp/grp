// Copyright 2024 feraxhp
// Licensed under the MIT License;

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) pconf: String,
    pub(crate) user: String,
    pub(crate) token: String,
    pub(crate) endpoint: String,
}

impl Config {
    pub fn new(
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
}
