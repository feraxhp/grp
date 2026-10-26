use mynox::Error;

use crate::common::structs::RequestType;
use crate::animation::Animation;
use crate::platform::Platform;
use crate::config::Config;
use crate::structs::User;


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
    let url = format!("{}/orgs", platform.get_base_url(config));
    let json = serde_json::json!({
        "username": name,
    });
    
    animation.change_message("creating org ...");
    let result = platform.post(url, true, config, &json).await?;
    
    animation.change_message("unwraping response ...");
    let base_message = "Error creating orgs";
    let result = platform.unwrap(result, base_message, config, &RequestType::CreateOrg).await?;
    
    User::from_text(&result, platform)
}
