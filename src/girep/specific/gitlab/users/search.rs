use serde_json::Value;

use crate::girep::error::structs::Error;
use crate::girep::common::structs::{Context, RequestType};
use crate::girep::json::JSON;
use crate::girep::platform::Platform;
use crate::girep::common::users::structs::User;
use crate::girep::config::Config;


pub async fn by_name(platform: &Platform, name: &String, conf: &Config) -> Result<Option<User>, Error> {
    let url = match platform {
        Platform::Gitlab => {
            format!("{}/users?username={}", platform.get_base_url(&conf.endpoint), name)
        },
        _ => unimplemented!("This platform is not supported for fetching users by name")
    };
    
    let result = platform.get(url, true, conf).await?;
    
    let context = Context {
        request_type: RequestType::UserList,
        owner: None, repo: None, additional: None,
    };
    
    let base = "Failed during fetch of logged user";
    let text = platform.unwrap(result, base,conf, context).await?;
    
    let json: Value = JSON::from_str(&text)?;
    
    let users = json.as_array().unwrap();
    
    if users.is_empty() { return Ok(None) }
    
    let user = users.iter().find(|u| u["username"].as_str().unwrap() == name);
    if let Some(user) = user {
        let user = User {
            id: user["id"].as_str().unwrap().to_string(),
            name: user["username"].as_str().unwrap().to_string(),
            path: None 
        };
        return Ok(Some(user))
    }
    
    Ok(None)
}
