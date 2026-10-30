use super::*;

use serde_json::json;
use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::common::test::user::gitlab::gitlab_user;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;


#[tokio::test]
async fn list_gitlab_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", json!({
            "id": 123, 
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_user" },
            "path_with_namespace": "gitlab_user/Hello-Gitlab"
        }), auth!(gl)),
        get!(200, "/api/v4/projects/123/issues", value!("issue/list.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_issues(None::<String>, &"Hello-Gitlab", &config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug in Gitlab");
    assert_eq!(issues[0].number, 1);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_gitlab_issues_for_repo() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_group%2FHello-Gitlab", json!({
            "id": 123, 
            "name": "Hello-Gitlab",
            "namespace": { "full_path": "gitlab_group" },
            "path_with_namespace": "gitlab_group/Hello-Gitlab"
        }), auth!(gl)),
        get!(200, "/api/v4/projects/123/issues", value!("issue/list.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_issues(Some("gitlab_group"), &"Hello-Gitlab", &config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug in Gitlab");
    assert_eq!(issues[0].number, 1);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_gitlab_user_assigned_issues() -> Result<(), mynox::Report> {
    let server = server!(
        gitlab_user(),
        get!(200, "/api/v4/issues", value!("issue/list.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_user_assigned_issues(&config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug in Gitlab");
    assert_eq!(issues[0].number, 1);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_gitlab_all_user_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/issues", value!("issue/list.gl.json"), auth!(gl)).query("scope", "assigned_to_me"),
        get!(200, "/api/v4/issues", value!("issue/list.gl.json"), auth!(gl)).query("scope", "created_by_me"),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_all_user_issues(&config, &an).await?;
    tokio::pin!(stream);
    let mut issues = vec![];
    while let Some(item) = stream.next().await {
        issues.extend(item?.result);
    }

    assert_eq!(issues.len(), 2);
    assert_eq!(&issues[0].title, "Found a bug in Gitlab");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_gitlab_all_involved_user_issues() -> Result<(), mynox::Report> {
    let todo_issue = json!([
        {
            "target_type": "Issue",
            "target": value!("issue/list.gl.json")[0]
        }
    ]);
    let server = server!(
        get!(200, "/api/v4/todos", todo_issue, auth!(gl)),
        get!(200, "/api/v4/issues", value!("issue/list.gl.json"), auth!(gl)).query("scope", "assigned_to_me"),
        get!(200, "/api/v4/issues", value!("issue/list.gl.json"), auth!(gl)).query("scope", "created_by_me"),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_all_involved_user_issues(&config, &an).await?;
    tokio::pin!(stream);
    let mut issues = vec![];
    while let Some(item) = stream.next().await {
        issues.extend(item?.result);
    }

    assert_eq!(issues.len(), 3);
    assert_eq!(&issues[0].title, "Found a bug in Gitlab");
    server.verify().await;
    Ok(())
}
