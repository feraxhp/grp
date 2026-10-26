use mynox::Error;
use urlencoding::encode;

use crate::common::structs::RequestType;
use crate::platform::Platform;
use crate::config::Config;
use crate::structs::User;


pub async fn premanently_remove(platform: &Platform, user: &User, config: &Config) -> Result<(), Error> {
    let user = user.clone();
    let id = user.id;
    let path = user.path.unwrap();
    let path = encode(&path);
    
    let url = format!("{}/groups/{}?full_path={}&permanently_remove=true", platform.get_base_url(config), id, path);
    let result = platform.delete(&url, config).await?;
    
    let context = RequestType::DeleteOrg; // This sould be permanent
    let base_message = "Error deleting group";
    
    if result.status().as_u16() == 202 { return Ok(()); }
    
    Err(platform.unwrap(result, base_message, config, &context).await.unwrap_err())
}
