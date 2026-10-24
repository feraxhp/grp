use super::*;

use futures::StreamExt;
use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::get;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;

#[tokio::test]
async fn list_gitlab_orgs() {
    let server = server!(
        get!(200, "/api/v4/groups", value!("org.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let stream = Gitlab.list_orgs(&config, &an);
    tokio::pin!(stream);
    let orgs = stream.next().await.unwrap().unwrap();

    assert_eq!(orgs.len(), 1);
    assert_eq!(&orgs[0].name, "gitlab_group");
    assert_eq!(&orgs[0].id, "100");
}
