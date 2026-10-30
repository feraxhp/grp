use super::*;

use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::common::test::user::forgejo::forgejo_user;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Forgejo;
use crate::animation;


#[tokio::test]
async fn list_forgejo_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/repos/forgejo_user/Hello-Forgejo/issues", value!("issue/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_issues(None::<String>, &"Hello-Forgejo", &config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug in Forgejo");
    assert_eq!(issues[0].number, 1);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_forgejo_issues_for_repo() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/repos/forgejo_org/Hello-Forgejo/issues", value!("issue/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_issues(Some("forgejo_org"), &"Hello-Forgejo", &config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug in Forgejo");
    assert_eq!(issues[0].number, 1);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_forgejo_user_assigned_issues() -> Result<(), mynox::Report> {
    let server = server!(
        forgejo_user(),
        get!(200, "/api/v1/repos/issues/search", value!("issue/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_user_assigned_issues(&config, &an).await?;
    tokio::pin!(stream);
    let issues = stream.next().await.unwrap()?.result;

    assert_eq!(issues.len(), 1);
    assert_eq!(&issues[0].title, "Found a bug in Forgejo");
    assert_eq!(issues[0].number, 1);
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_forgejo_all_user_issues() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/repos/issues/search", value!("issue/list.fj.json"), auth!(fj)).query("assigned", "true"),
        get!(200, "/api/v1/repos/issues/search", value!("issue/list.fj.json"), auth!(fj)).query("created", "true"),
        get!(200, "/api/v1/repos/issues/search", value!("issue/list.fj.json"), auth!(fj)).query("mentioned", "true"),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_all_user_issues(&config, &an).await?;
    tokio::pin!(stream);
    let mut issues = vec![];
    while let Some(item) = stream.next().await {
        issues.extend(item?.result);
    }

    assert_eq!(issues.len(), 3);
    assert_eq!(&issues[0].title, "Found a bug in Forgejo");
    server.verify().await;
    Ok(())
}
