// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::config::Config;
use crate::girep::repo::Repo;
use async_trait::async_trait;
use hyper::HeaderMap;

#[async_trait]
pub(crate) trait Platform: Send + Sync {
    fn new(config: Config) -> Self where Self: Sized;
    fn get_auth_header(token: String) -> HeaderMap where Self: Sized;
    async fn list_repos(&self, owner: Option<String>) -> Vec<Repo>;
    async fn create_repo(&self, owner: String, repo: Repo) -> Repo;
}
