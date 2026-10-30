use super::*;

use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Forgejo;
use crate::animation;


#[tokio::test]
async fn get_forgejo_issue() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/repos/forgejo_user/Hello-Forgejo/issues/1", value!("issue/view.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let issue = Forgejo.issue(None::<String>, &"Hello-Forgejo", &1, &config, &an).await?;

    assert_eq!(&issue.title, "Found a bug in Forgejo");
    assert_eq!(issue.number, 1);
    assert_eq!(&issue.author.name, "forgejo_user");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn get_forgejo_issue_with_owner() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/repos/forgejo_user/Hello-Forgejo/issues/1", value!("issue/view.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let issue = Forgejo.issue(Some("forgejo_user"), &"Hello-Forgejo", &1, &config, &an).await?;

    assert_eq!(&issue.title, "Found a bug in Forgejo");
    assert_eq!(issue.number, 1);
    assert_eq!(&issue.author.name, "forgejo_user");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_forgejo_issue_comments() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/repos/forgejo_user/Hello-Forgejo/issues/1/comments", value!("issue/comment.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_issue_comments(None::<String>, &"Hello-Forgejo", &1, &config, &an).await?;
    tokio::pin!(stream);
    let comments = stream.next().await.unwrap()?.result;

    assert_eq!(comments.len(), 1);
    assert_eq!(&comments[0].body, "Thanks for reporting this issue! We're looking into it.");
    assert_eq!(&comments[0].author.name, "forgejo_user");
    server.verify().await;
    Ok(())
}
