use mynox::Error;
use urlencoding::encode;

use crate::JSON;
use crate::common::structs::RequestType;
use crate::common::traits::Convert;
use crate::platform::Platform;
use crate::config::Config;
use crate::specific::gitlab::parser::Group;
use crate::structs::User;

pub async fn get_group_with_path(platform: &Platform, path: &str, config: &Config) -> Result<User, Error> {
    let path  = encode(path);
    let url = format!("{}/groups/{}", platform.get_base_url(config), path);
    
    get_group(platform, &url, config, RequestType::ListOrg { way: format!("path: {path}") }).await
}

pub async fn get_group_by_id(platform: &Platform, id: &str, config: &Config) -> Result<User, Error> {
    let url = format!("{}/groups/{}", platform.get_base_url(config), id);
    
    get_group(platform, &url, config, RequestType::ListOrg { way: format!("by_id: {id}") }).await
}

pub async fn get_group(platform: &Platform, url: &str, config: &Config, request_type: RequestType) -> Result<User, Error> {
    let result = platform.get(url, true, config).await?;
    
    let base_message = "Error geting the group id";
    let result = platform.unwrap(result, base_message, config, &request_type).await?;
    
    let pj: Group = JSON::from_str(&result)?;
    
    Ok(pj.convert())
}
