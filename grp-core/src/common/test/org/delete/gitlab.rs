use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::get;
use crate::delete;
use crate::value;
use crate::Platform::Gitlab;
use crate::animation;

#[tokio::test]
async fn delete_gitlab_org() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/groups/gitlab_group", value!("../create/org.gl.json"), auth!(gl)),
        delete!(202, "/api/v4/groups/100", auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    Gitlab.delete_org("gitlab_group", &config, false, &an).await?;
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn delete_gitlab_org_permanently() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/groups/gitlab_group", value!("../create/org.gl.json"), auth!(gl)),
        delete!(202, "/api/v4/groups/100", auth!(gl)).query("permanently_remove", "true"),
        delete!(202, "/api/v4/groups/100", auth!(gl)),
        get!(200, "/api/v4/groups/100", value!("../create/org.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    Gitlab.delete_org("gitlab_group", &config, true, &an).await?;
    server.verify().await;
    Ok(())
}
