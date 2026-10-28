use super::*;

use serde_json::json;
use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;


#[tokio::test]
async fn get_gitlab_issue() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123, 
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        get!(200, "/api/v4/projects/123/issues/1", value!("issue.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let issue = Gitlab.issue(None::<String>, &"Hello-Gitlab", &1, &config, &an).await?;

    assert_eq!(&issue.title, "Found a bug in Gitlab");
    assert_eq!(issue.number, 1);
    assert_eq!(&issue.author.name, "gitlab_user");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn get_gitlab_issue_with_owner() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123, 
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        get!(200, "/api/v4/projects/123/issues/1", value!("issue.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let issue = Gitlab.issue(Some("gitlab_user"), &"Hello-Gitlab", &1, &config, &an).await?;

    assert_eq!(&issue.title, "Found a bug in Gitlab");
    assert_eq!(issue.number, 1);
    assert_eq!(&issue.author.name, "gitlab_user");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_gitlab_issue_comments() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123, 
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        get!(200, "/api/v4/projects/123/issues/1/notes", value!("comment.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_issue_comments(None::<String>, &"Hello-Gitlab", &1, &config, &an).await?;
    tokio::pin!(stream);
    let comments = stream.next().await.unwrap()?.result;

    assert_eq!(comments.len(), 1);
    assert_eq!(&comments[0].body, "Text of the comment");
    assert_eq!(&comments[0].author.name, "gitlab_user");
    server.verify().await;
    Ok(())
}
