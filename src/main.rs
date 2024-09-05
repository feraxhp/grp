// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
use crate::girep::base::RepoProvider;
use crate::girep::config::Config;

#[tokio::main]
async fn main() {

    let config: Config = Config::new(
        "feraxhp".to_string(),
        "<gitea token>".to_string(),
        "<gitea url>".to_string(),
    );

    let gitea = girep::repos::gitea::Gitea::new(config);

    let reps = gitea.list_repos(None).await;

    show!(reps);

    let config: Config = Config::new(
        "feraxhp".to_string(),
        "<github token>".to_string(),
        "api.github.com".to_string(),
    );

    let github = girep::repos::github::Github::new(config);

    let reps = github.list_repos(None).await;

    show!(reps);
}
