use super::*;

use serde_json::json;
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
async fn get_forgejo_repo() -> Result<(), mynox::Report> {
    let server = server!(
        forgejo_user(),
        get!(200, "/api/v1/user/repos", value!("repo/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_repos(None::<String>, &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-Forgejo");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_forgejo_repos_for_org() -> Result<(), mynox::Report> {
    let server = server!(
        forgejo_user(),
        get!(200, "/api/v1/user/orgs", value!("org/list.fj.json"), auth!(fj)),
        get!(200, "/api/v1/orgs/forgejo_org/repos", value!("repo/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_repos(Some("forgejo_org"), &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;
    
    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-Forgejo");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_forgejo_repos_for_unlogged_user() -> Result<(), mynox::Report> {
    let server = server!(
        forgejo_user(),
        get!(200, "/api/v1/user/orgs", json!([]), auth!(fj)),
        get!(200, "/api/v1/users/other_user/repos", value!("repo/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_repos(Some("other_user"), &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-Forgejo");
    server.verify().await;
    Ok(())
}
