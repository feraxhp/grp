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
async fn list_forgejo_orgs() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v1/user/orgs", value!("org/list.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let stream = Forgejo.list_orgs(&config, &an);
    tokio::pin!(stream);
    let orgs = stream.next().await.unwrap()?.result;

    assert_eq!(orgs.len(), 1);
    assert_eq!(&orgs[0].name, "forgejo_org");
    server.verify().await;
    Ok(())
}
