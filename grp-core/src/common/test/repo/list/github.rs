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
async fn get_github_repo() -> Result<(), mynox::Report> {
    let server = server!(
        github_user(),
        get!(200, "/user/repos", value!("repo.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_repos(None::<String>, &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-World");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_repos_for_org() -> Result<(), mynox::Report> {
    let server = server!(
        github_user(),
        get!(200, "/user/orgs", value!("../../org/list/org.gh.json"), auth!(gh)),
        get!(200, "/orgs/github/repos", value!("repo.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_repos(Some("github"), &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-World");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_github_repos_for_unlogged_user() -> Result<(), mynox::Report> {
    let server = server!(
        github_user(),
        get!(200, "/user/orgs", json!([]), auth!(gh)),
        get!(200, "/users/torvalds/repos", value!("repo.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_repos(Some("torvalds"), &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-World");
    server.verify().await;
    Ok(())
}
