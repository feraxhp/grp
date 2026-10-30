use super::*;

use serde_json::json;
use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::common::test::user::github::github_user;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Github;
use crate::animation;


#[tokio::test]
async fn list_github_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/repos/octocat/Hello-World/issues", value!("issue/list.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_issues(None::<String>, &"Hello-World", &config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug");
    assert_eq!(issues[0].number, 1347);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_issues_for_repo() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/repos/other_user/Hello-World/issues", value!("issue/list.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_issues(Some("other_user"), &"Hello-World", &config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug");
    assert_eq!(issues[0].number, 1347);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_user_assigned_issues() -> Result<(), mynox::Report> {
    let server = server!(
        github_user(),
        get!(200, "/user/issues", value!("issue/list.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_user_assigned_issues(&config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug");
    assert_eq!(issues[0].number, 1347);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_all_user_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/issues", value!("issue/list.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_all_user_issues(&config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug");
    assert_eq!(issues[0].number, 1347);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_all_involved_user_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/search/issues", json!({ "items": value!("issue/list.gh.json") }), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_all_involved_user_issues(&config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug");
    assert_eq!(issues[0].number, 1347);
    server.verify().await;
    Ok(())
}
