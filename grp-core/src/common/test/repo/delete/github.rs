use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::delete;
use crate::Platform::Github;
use crate::animation;

#[tokio::test]
async fn delete_github_repo() -> Result<(), mynox::Report> {
    let server = server!(
        delete!(204, "/repos/octocat/Hello-World", auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    Github.delete_repo("octocat", "Hello-World", &config, false, &an).await?;
    server.verify().await;
    Ok(())
}
