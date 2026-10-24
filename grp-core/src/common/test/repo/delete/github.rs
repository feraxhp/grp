use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::delete;
use crate::Platform::Github;
use crate::animation;

#[tokio::test]
async fn delete_github_repo() {
    let server = server!(
        delete!(204, "/repos/octocat/Hello-World", auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let result = Github.delete_repo("octocat", "Hello-World", &config, false, &an).await;

    assert!(result.is_ok());
}
