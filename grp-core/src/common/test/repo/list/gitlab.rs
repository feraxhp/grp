use super::*;

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
async fn get_gitlab_repo() -> Result<(), mynox::Report> {
    let server = server!(
        gitlab_user(),
        get!(200, "/api/v4/projects", value!("repo/list.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_repos(None::<String>, &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-Gitlab");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn list_gitlab_repos_for_org() -> Result<(), mynox::Report> {
    let server = server!(
        gitlab_user(),
        get!(200, "/api/v4/groups", value!("org/list.gl.json"), auth!(gl)),
        get!(200, "/api/v4/groups/100/projects", value!("repo/list.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_repos(Some("gitlab_group"), &config, &an).await?;
    tokio::pin!(stream);
    let repos = stream.next().await.unwrap()?.result;

    assert_eq!(repos.len(), 1);
    assert_eq!(&repos[0].name, "Hello-Gitlab");
    server.verify().await;
    Ok(())
}
