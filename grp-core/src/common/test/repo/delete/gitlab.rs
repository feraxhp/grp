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
async fn delete_gitlab_repo() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", value!("../create/repo.gl.json"), auth!(gl)),
        delete!(202, "/api/v4/projects/123", auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    Gitlab.delete_repo("gitlab_user", "Hello-Gitlab", &config, false, &an).await?;
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn delete_gitlab_repo_permanently() -> Result<(), mynox::Report> {
    let server = server!(
        get!(200, "/api/v4/projects/gitlab_user%2FHello-Gitlab", value!("../create/repo.gl.json"), auth!(gl)),
        delete!(202, "/api/v4/projects/123", auth!(gl)).query("permanently_remove", "true"),
        delete!(202, "/api/v4/projects/123", auth!(gl)),
        get!(200, "/api/v4/projects/123", value!("../create/repo.gl.json"), auth!(gl)),
    );
    
    let config = config!(gl server);
    let an = Box::new(animation::None);
    
    Gitlab.delete_repo("gitlab_user", "Hello-Gitlab", &config, true, &an).await?;
    server.verify().await;
    Ok(())
}
