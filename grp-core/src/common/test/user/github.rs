use crate::Config;
use crate::config;
use crate::server;
use crate::Platform::Github;

pub use super::*;

pub fn github_user() -> Endpoint<4> { get!(200, "/user", value!("user.gh.json"), auth!(gh)) }

#[tokio::test]
async fn github_authenticated_user() {
    let server = server!(github_user());
    let config = config!(gh server);
    
    let user = Github.get_logged_user(&config).await.unwrap();
    
    assert_eq!(&user.name, "octocat");
}
