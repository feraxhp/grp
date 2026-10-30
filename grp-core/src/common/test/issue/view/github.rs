use super::*;

use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Github;
use crate::animation;


#[tokio::test]
async fn get_github_issue() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/repos/octocat/Hello-World/issues/1347", value!("issue/view.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let issue = Github.issue(None::<String>, &"Hello-World", &1347, &config, &an).await?;

    assert_eq!(&issue.title, "Found a bug");
    assert_eq!(issue.number, 1347);
    assert_eq!(&issue.author.name, "octocat");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn get_github_issue_with_owner() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/repos/octocat/Hello-World/issues/1347", value!("issue/view.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let issue = Github.issue(Some("octocat"), &"Hello-World", &1347, &config, &an).await?;

    assert_eq!(&issue.title, "Found a bug");
    assert_eq!(issue.number, 1347);
    assert_eq!(&issue.author.name, "octocat");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_issue_comments() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/repos/octocat/Hello-World/issues/1347/comments", value!("issue/comment.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_issue_comments(None::<String>, &"Hello-World", &1347, &config, &an).await?;
    tokio::pin!(stream);
    let comments = stream.next().await.unwrap()?.result;

    assert_eq!(comments.len(), 1);
    assert_eq!(&comments[0].body, "What version of Safari were you using when you observed this bug?");
    assert_eq!(&comments[0].author.name, "octocat");
    server.verify().await;
    Ok(())
}
