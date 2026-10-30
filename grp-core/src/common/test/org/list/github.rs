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
async fn list_github_orgs() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/user/orgs", value!("org/list.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let stream = Github.list_orgs(&config, &an);
    tokio::pin!(stream);
    let orgs = stream.next().await.unwrap()?.result;

    assert_eq!(orgs.len(), 1);
    assert_eq!(&orgs[0].name, "github");
    server.verify().await;
    Ok(())
}
