use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::config;
use crate::server;
use crate::post;
use crate::value;
use crate::Platform::Forgejo;
use crate::animation;

#[tokio::test]
async fn create_forgejo_org() {
    let payload = json!({
        "username": "forgejo_org",
    });
    
    let server = server!(
        post!(201, "/api/v1/orgs", payload, value!("org.fj.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let (orgs, errors) = Forgejo.create_org("forgejo_org", &config, false, &an).await;

    assert!(errors.is_empty());
    assert_eq!(orgs.len(), 1);
    assert_eq!(&orgs[0].name, "forgejo_org");
}
