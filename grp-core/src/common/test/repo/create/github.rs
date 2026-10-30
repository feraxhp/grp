use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::common::test::user::github::github_user;
use crate::common::structs::Repo;
use crate::config;
use crate::server;
use crate::post;
use crate::get;
use crate::value;
use crate::Platform::Github;
use crate::animation;

#[tokio::test]
async fn create_github_repo() -> Result<(), mynox::Report> {
    let payload = json!({
        "name": "Hello-World",
        "description": "This your first repo!",
        "private": false,
    });
    
    let server = server!(
        github_user(),
        post!(201, "/user/repos", payload, value!("repo.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let repo = Repo {
        name: "Hello-World".to_string(),
        path: "".to_string(),
        private: Some(false),
        url: "".to_string(),
        git: "".to_string(),
        description: Some("This your first repo!".to_string()),
    };
    
    let created = Github.create_repo(None::<String>, repo, &config, &an).await?;
    
    assert_eq!(&created.name, "Hello-World");
    assert_eq!(&created.path, "octocat/Hello-World");
    assert_eq!(created.private, Some(false));
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn create_github_repo_for_org() -> Result<(), mynox::Report> {
    let payload = json!({
        "name": "Hello-World",
        "description": "This your first repo!",
        "private": false,
    });
    
    let server = server!(
        github_user(),
        get!(200, "/user/orgs", value!("../../org/list/org.gh.json"), auth!(gh)),
        post!(201, "/orgs/github/repos", payload, value!("repo.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let repo = Repo {
        name: "Hello-World".to_string(),
        path: "".to_string(),
        private: Some(false),
        url: "".to_string(),
        git: "".to_string(),
        description: Some("This your first repo!".to_string()),
    };
    
    let created = Github.create_repo(Some("github"), repo, &config, &an).await?;
    
    assert_eq!(&created.name, "Hello-World");
    assert_eq!(&created.path, "octocat/Hello-World");
    server.verify().await;
    Ok(())
}
