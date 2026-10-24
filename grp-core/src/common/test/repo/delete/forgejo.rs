use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::delete;
use crate::Platform::Forgejo;
use crate::animation;

#[tokio::test]
async fn delete_forgejo_repo() {
    let server = server!(
        delete!(204, "/api/v1/repos/forgejo_user/Hello-Forgejo", auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let result = Forgejo.delete_repo("forgejo_user", "Hello-Forgejo", &config, false, &an).await;

    assert!(result.is_ok());
}
