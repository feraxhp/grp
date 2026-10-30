use crate::Config;
use crate::config;
use crate::server;
use crate::Platform::Gitlab;

pub use super::*;

pub fn gitlab_user() -> Endpoint<2> { get!(200, "/api/v4/user", value!("user/user.gl.json"), auth!(gl)) }

#[tokio::test]
async fn gitlab_authenticated_user() -> Result<(), mynox::Report> {
    let server = server!(gitlab_user());
    let config = config!(gl server);
    
    let user = Gitlab.get_logged_user(&config).await?;
    
    assert_eq!(&user.name, "gitlab_user");
    assert_eq!(&user.id, "1");
    server.verify().await;
    Ok(())
}
