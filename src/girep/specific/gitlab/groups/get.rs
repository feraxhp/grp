use urlencoding::encode;

use crate::girep::common::structs::{Context, RequestType};
use crate::girep::common::users::structs::User;
use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;
use crate::girep::config::Config;

pub async fn get_group_with_path(platform: &Platform, path: &str, config: &Config) -> Result<User, Error> {
    let path  = encode(&path);
    let url = format!("{}/groups/{}", platform.get_base_url(&config.endpoint), path);
    
    get_group(platform, &url, config).await
}

pub async fn get_group_by_id(platform: &Platform, id: &str, config: &Config) -> Result<User, Error> {
    let url = format!("{}/groups/{}", platform.get_base_url(&config.endpoint), id);
    
    get_group(platform, &url, config).await
}

pub async fn get_group(platform: &Platform, url: &str, config: &Config) -> Result<User, Error> {
    let result = platform.get(url, true, config).await?;
    
    let base_message = "Error geting the project id";
    
    let context = Context {
        request_type: RequestType::RepositoryDetails,
        owner: None, repo: None, additional: None
    };
    
    let result = platform.unwrap(result, base_message, config, context).await?;
    
    let pj: User = User::from_text(&result, platform)?;
    
    Ok(pj)
}