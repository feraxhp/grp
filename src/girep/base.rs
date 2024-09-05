// Copyright 2024 feraxhp
// Licensed under the MIT License;

use hyper::HeaderMap;
use crate::girep::config::Config;
use crate::girep::repo::Repo;

pub(crate) trait RepoProvider {
    fn new(config: Config) -> Self;
    fn get_auth_header(token: String) -> HeaderMap;
    async fn list_repos(&self, owner: Option<String>) -> Vec<Repo>;
}
