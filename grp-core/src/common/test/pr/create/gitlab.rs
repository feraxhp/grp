use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::common::structs::BranchIdentifier;
use crate::common::structs::PullRequestOptions;
use crate::config;
use crate::server;
use crate::post;
use crate::get;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;

#[tokio::test]
async fn create_gitlab_pr() -> Result<(), mynox::Report> {
    let payload = json!({
        "title": "test1",
        "description": "fixed login page css paddings",
        "source_branch": "test1",
        "target_branch": "main",
        "target_project_id": 123,
        "allow_collaboration": false,
    });
    
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123,
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        post!(201, "/api/v4/projects/gitlab_user%2FHello-Gitlab/merge_requests", payload, value!("pr/create.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: "gitlab_user".to_string(),
            repo: "Hello-Gitlab".to_string(),
            branch: "main".to_string(),
        },
        origin: BranchIdentifier {
            owner: "gitlab_user".to_string(),
            repo: "Hello-Gitlab".to_string(),
            branch: "test1".to_string(),
        },
        title: "test1".to_string(),
        body: "fixed login page css paddings".to_string(),
        draft: false,
        maintainer_can_modify: false,
    };
    
    let created = Gitlab.create_pr(&options, &config, &an).await?;
    
    assert_eq!(&created.title, "test1");
    assert_eq!(&created.body, "fixed login page css paddings");
    assert_eq!(created.number, 1);
    assert_eq!(&created.author.name, "admin");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn create_gitlab_pr_from_fork() -> Result<(), mynox::Report> {
    let payload = json!({
        "title": "test1",
        "description": "fixed login page css paddings",
        "source_branch": "test1",
        "target_branch": "main",
        "target_project_id": 123,
        "allow_collaboration": true,
    });
    
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123,
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        post!(201, "/api/v4/projects/other_user%2FHello-Gitlab/merge_requests", payload, value!("pr/create.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: "gitlab_user".to_string(),
            repo: "Hello-Gitlab".to_string(),
            branch: "main".to_string(),
        },
        origin: BranchIdentifier {
            owner: "other_user".to_string(),
            repo: "Hello-Gitlab".to_string(),
            branch: "test1".to_string(),
        },
        title: "test1".to_string(),
        body: "fixed login page css paddings".to_string(),
        draft: false,
        maintainer_can_modify: true,
    };
    
    let created = Gitlab.create_pr(&options, &config, &an).await?;
    
    assert_eq!(&created.title, "test1");
    assert_eq!(created.number, 1);
    server.verify().await;
    Ok(())
}
