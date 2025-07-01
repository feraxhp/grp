use urlencoding::encode;

use crate::girep::common::structs::{Context, RequestType};
use crate::girep::common::users::structs::User;
use crate::girep::config::Config;
use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;


pub async fn premanently_remove(platform: &Platform, user: &User, config: &Config) -> Result<(), Error> {
    let user = user.clone();
    let id = user.id;
    let path = user.path.unwrap();
    let path = encode(&path);
    
    let url = format!("{}/groups/{}?full_path={}&permanently_remove=true", platform.get_base_url(&config.endpoint), &id, path);
    let result = platform.delete(&url, config).await?;
    
    let context = Context {
        request_type: RequestType::DeletePermanent,
        owner: None, repo: Some(id),
        additional: None
    };
    
    let base_message = "Error deleting group";
    
    if result.status().as_u16() == 202 { return Ok(()); }
    
    Err(platform.unwrap(result, base_message, &config, context).await.unwrap_err())
}