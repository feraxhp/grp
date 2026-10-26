use mynox::Error;

use crate::common::structs::RequestType;
use crate::common::traits::Convert;
use crate::platform::Platform;
use crate::specific::gitlab;
use crate::config::Config;
use crate::structs::User;
use crate::json::JSON;


pub async fn by_name(platform: &Platform, name: &String, conf: &Config) -> Result<Option<User>, Error> {
    let url = match platform {
        Platform::Gitlab => {
            format!("{}/users?username={}", platform.get_base_url(conf), name)
        },
        _ => unimplemented!("This platform is not supported for fetching users by name")
    };
    
    let result = platform.get(url, true, conf).await?;
    
    let base = "Failed during fetch of logged user";
    let text = platform.unwrap(result, base,conf, &RequestType::ListUser).await?;
    
    let users: Vec<gitlab::parser::User> = JSON::from_str(&text)?;
    if users.is_empty() { return Ok(None) }
    
    let user = users.iter().find(|u| u.username.as_str() == name);
    if let Some(user) = user {
        let user = user.convert();
        return Ok(Some(user))
    }
    
    Ok(None)
}
