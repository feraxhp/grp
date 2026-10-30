use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::common::test::user::forgejo::forgejo_user;
use crate::common::structs::Repo;
use crate::config;
use crate::server;
use crate::post;
use crate::get;
use crate::value;
use crate::Platform::Forgejo;
use crate::animation;

#[tokio::test]
async fn create_forgejo_repo() -> Result<(), mynox::Report> {
    let payload = json!({
        "name": "Hello-Forgejo",
        "description": "My first Forgejo repo!",
        "private": false,
    });
    
    let server = server!(
        forgejo_user(),
        post!(201, "/api/v1/user/repos", payload, value!("repo/create.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let repo = Repo {
        name: "Hello-Forgejo".to_string(),
        path: "".to_string(),
        private: Some(false),
        url: "".to_string(),
        git: "".to_string(),
        description: Some("My first Forgejo repo!".to_string()),
    };
    
    let created = Forgejo.create_repo(None::<String>, repo, &config, &an).await?;
    
    assert_eq!(&created.name, "Hello-Forgejo");
    assert_eq!(&created.path, "forgejo_user/Hello-Forgejo");
    assert_eq!(created.private, Some(false));
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn create_forgejo_repo_for_org() -> Result<(), mynox::Report> {
    let payload = json!({
        "name": "Hello-Forgejo",
        "description": "My first Forgejo repo!",
        "private": false,
    });
    
    let server = server!(
        forgejo_user(),
        get!(200, "/api/v1/user/orgs", value!("org/list.fj.json"), auth!(fj)),
        post!(201, "/api/v1/orgs/forgejo_org/repos", payload, value!("repo/create.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let repo = Repo {
        name: "Hello-Forgejo".to_string(),
        path: "".to_string(),
        private: Some(false),
        url: "".to_string(),
        git: "".to_string(),
        description: Some("My first Forgejo repo!".to_string()),
    };
    
    let created = Forgejo.create_repo(Some("forgejo_org"), repo, &config, &an).await?;
    
    assert_eq!(&created.name, "Hello-Forgejo");
    assert_eq!(&created.path, "forgejo_user/Hello-Forgejo");
    server.verify().await;
    Ok(())
}
