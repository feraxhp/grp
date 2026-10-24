use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::delete;
use crate::Platform::Github;
use crate::animation;

#[tokio::test]
async fn delete_github_org() {
    let server = server!(
        delete!(202, "/orgs/github", auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let result = Github.delete_org("github", &config, false, &an).await;

    assert!(result.is_ok());
}
