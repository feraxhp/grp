use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::delete;
use crate::Platform::Github;
use crate::animation;

#[tokio::test]
async fn delete_github_org() -> Result<(), mynox::Report> {
    let server = server!(
        delete!(202, "/orgs/github", auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    Github.delete_org("github", &config, false, &an).await?;
    server.verify().await;
    Ok(())
}
