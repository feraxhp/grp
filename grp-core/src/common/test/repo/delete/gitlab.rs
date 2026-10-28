use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::get;
use crate::delete;
use crate::Platform::Gitlab;
use crate::animation;

#[tokio::test]
async fn delete_gitlab_repo() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123, 
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        delete!(202, "/api/v4/projects/123", auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    Gitlab.delete_repo("gitlab_user", "Hello-Gitlab", &config, false, &an).await?;
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn delete_gitlab_repo_permanently() -> Result<(), mynox::Report> {
    let project = json!({
        "id": 123, 
        "name": "Hello-Gitlab",
        "namespace": { "full_path": "gitlab_user" },
        "path_with_namespace": "gitlab_user/Hello-Gitlab"
    });
    
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", project.clone(), auth!(gl)),
        delete!(202, "/api/v4/projects/123", auth!(gl)).query("permanently_remove", "true"),
        delete!(202, "/api/v4/projects/123", auth!(gl)),
        get!(200, "/api/v4/projects/123", project, auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    Gitlab.delete_repo("gitlab_user", "Hello-Gitlab", &config, true, &an).await?;
    server.verify().await;
    Ok(())
}
