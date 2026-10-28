use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::config;
use crate::faild;
use crate::server;
use crate::get;
use crate::post;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;

#[tokio::test]
async fn create_gitlab_org() -> Result<(), mynox::Report> {
    let payload = json!({
        "parent_id": null,
        "name": "gitlab_group",
        "path": "gitlab_group",
    });
    
    let server = server!(
        get!(200, "/api/v4/groups", json!([]), auth!(gl)),
        post!(201, "/api/v4/groups", payload, value!("org.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    let (orgs, errors) = Gitlab.create_org("gitlab_group", &config, false, &an).await;
    
    faild!(errors);
    
    assert_eq!(orgs.len(), 1);
    assert_eq!(&orgs[0].name, "gitlab_group");
    assert_eq!(&orgs[0].id, "100");
    server.verify().await;
    Ok(())
}
