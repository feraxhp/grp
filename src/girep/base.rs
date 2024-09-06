// Copyright 2024 feraxhp
// Licensed under the MIT License;

use hyper::HeaderMap;
use crate::girep::config::Config;
use crate::girep::repo::Repo;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Platform: Send + Sync {
    fn new(config: Config) -> Self where Self: Sized;
    fn get_auth_header(token: String) -> HeaderMap where Self: Sized;
    async fn list_repos(&self, owner: Option<String>) -> Vec<Repo>;
}
