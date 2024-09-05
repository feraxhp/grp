// Copyright 2024 feraxhp
// Licensed under the MIT License;

pub(crate) struct Config {
    pub(crate) user: String,
    pub(crate) token: String,
    pub(crate) endpoint: String,
}

impl Config {
    pub(crate) fn new(
        user: String,
        token: String,
        endpoint: String
    ) -> Self {
        Config {
            user,
            token,
            endpoint
        }
    }
}
