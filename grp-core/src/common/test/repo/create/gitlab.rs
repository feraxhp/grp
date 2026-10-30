use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::common::test::user::gitlab::gitlab_user;
use crate::common::structs::Repo;
use crate::config;
use crate::server;
use crate::post;
use crate::get;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;

#[tokio::test]
async fn create_gitlab_repo() -> Result<(), mynox::Report> {
    let payload = json!({
        "path": "Hello-Gitlab",
        "description": "My first Gitlab project!",
        "visibility": "public",
    });
    
    let server = server!(
        gitlab_user(),
        post!(201, "/api/v4/projects", payload, value!("repo/create.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let repo = Repo {
        name: "Hello-Gitlab".to_string(),
        path: "".to_string(),
        private: Some(false),
        url: "".to_string(),
        git: "".to_string(),
        description: Some("My first Gitlab project!".to_string()),
    };
    
    let created = Gitlab.create_repo(None::<String>, repo, &config, &an).await?;
    
    assert_eq!(&created.name, "Hello-Gitlab");
    assert_eq!(&created.path, "gitlab_user/Hello-Gitlab");
    assert_eq!(created.private, Some(false));
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn create_gitlab_repo_for_org() -> Result<(), mynox::Report> {
    let payload = json!({
        "path": "Hello-Gitlab",
        "description": "My first Gitlab project!",
        "visibility": "public",
    });
    
    let server = server!(
        gitlab_user(),
        get!(200, "/api/v4/groups", value!("org/list.gl.json"), auth!(gl)),
        post!(201, "/api/v4/projects", payload, value!("repo/create.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let repo = Repo {
        name: "Hello-Gitlab".to_string(),
        path: "".to_string(),
        private: Some(false),
        url: "".to_string(),
        git: "".to_string(),
        description: Some("My first Gitlab project!".to_string()),
    };
    
    let created = Gitlab.create_repo(Some("gitlab_group"), repo, &config, &an).await?;
    
    assert_eq!(&created.name, "Hello-Gitlab");
    assert_eq!(&created.path, "gitlab_user/Hello-Gitlab");
    server.verify().await;
    Ok(())
}
