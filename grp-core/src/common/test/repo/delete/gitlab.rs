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
async fn delete_gitlab_repo() {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({"id": 123, "path_with_namespace": "gitlab_user/Hello-Gitlab"}), auth!(gl)),
        delete!(202, "/api/v4/projects/123", auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let result = Gitlab.delete_repo("gitlab_user", "Hello-Gitlab", &config, false, &an).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_gitlab_repo_permanently() {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({"id": 123, "path_with_namespace": "gitlab_user/Hello-Gitlab"}), auth!(gl)),
        delete!(202, "/api/v4/projects/123", auth!(gl)).query("permanently_remove", "true"),
        delete!(202, "/api/v4/projects/123", auth!(gl)),
        get!(200, "/api/v4/projects/123", json!({"id": 123, "path_with_namespace": "gitlab_user/Hello-Gitlab"}), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let result = Gitlab.delete_repo("gitlab_user", "Hello-Gitlab", &config, true, &an).await;

    assert!(result.is_ok());
}
