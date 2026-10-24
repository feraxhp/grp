use crate::Config;
use crate::config;
use crate::server;
use crate::Platform::Forgejo;

pub use super::*;

pub fn forgejo_user() -> Endpoint<2> { get!(200, "/api/v1/user", value!("user.fj.json"), auth!(fj)) }

#[tokio::test]
async fn forgejo_authenticated_user() {
    let server = server!(forgejo_user());
    let config = config!(fj server);
    
    let user = Forgejo.get_logged_user(&config).await.unwrap();
    
    assert_eq!(&user.name, "forgejo_user");
}
