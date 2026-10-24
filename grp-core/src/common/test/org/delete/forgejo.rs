use super::*;

use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::delete;
use crate::Platform::Forgejo;
use crate::animation;

#[tokio::test]
async fn delete_forgejo_org() {
    let server = server!(
        delete!(204, "/api/v1/orgs/forgejo_org", auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let result = Forgejo.delete_org("forgejo_org", &config, false, &an).await;

    assert!(result.is_ok());
}
