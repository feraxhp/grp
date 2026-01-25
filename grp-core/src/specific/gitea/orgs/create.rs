use crate::animation::Animation;
use crate::common::structs::{Context, RequestType};
use crate::common::users::structs::User;
use crate::platform::Platform;
use crate::error::structs::Error;
use crate::config::Config;


pub(crate) async fn create<A: Animation + ?Sized>(
    platform: &Platform, 
    name: &String, 
    config: &Config,
    animation: &Box<A>
) -> Result<User, Error> {
    assert!(
        matches!(platform, Platform::Gitea) || 
        matches!(platform, Platform::Codeberg) ||
        matches!(platform, Platform::Forgejo)
    );
    
    animation.change_message("generating url ...");
    let url = format!("{}/orgs", platform.get_base_url(&config.endpoint));
    let json = serde_json::json!({
        "username": name,
    });
    
    animation.change_message("creating org ...");
    let result = platform.post(url, true, config, &json).await?;
    
    animation.change_message("unwraping response ...");
    let context = Context {
        request_type: RequestType::CreateOrg,
        owner: None, repo: None,
        additional: Some(name.clone()),
    };
    
    let base_message = "Error creating orgs";
    let result = platform.unwrap(result, base_message, config, context).await?;
    
    User::from_text(&result, platform)
}