use std::future;

use serde_json::Value;
use futures::StreamExt;

use crate::json::JSON;
use crate::structs::{Context, RequestType};
use crate::{config::Config, platform::Platform};
use crate::common::users::structs::User;
use crate::error::structs::Error;


pub async fn by_full_path(name: &String, config: &Config) -> Result<Option<User>, Error> {
    let platform = Platform::Gitlab;
    let url = format!("{}/groups?search={}", platform.get_base_url(&config.endpoint), name);
    
    let context = Context {
        request_type: RequestType::ListOrg,
        owner: None,
        repo: None,
        additional: Some(format!("Error finding {} id", name)),
    };
    let mut user: Option<User> = None;
    
    let errors: Vec<Error> = platform.pagginate(url, config, context)
        .map(|result| -> Result<Vec<Value>, Error>{
            match result {
                Ok(s) => JSON::from_str(&s),
                Err(e) => Err(e),
            }
        })
        .take_while(|s| {
            let json = match s {
                Ok(s) => s,
                Err(_) => return future::ready(false),
            };
            
            if json.is_empty() { return future::ready(true) }
            
            match json.iter().find(|u| u["full_path"].as_str().unwrap() == name) {
                Some(json) => {
                    let user_ = User {
                        id: json["id"].as_u64().unwrap().to_string(),
                        name: json["name"].as_str().unwrap().to_string(),
                        path: Some(json["full_path"].as_str().unwrap().to_string()),
                    };
                    
                    user = Some(user_);
                    future::ready(false)
                },
                None => future::ready(true),
            }
        })
        .fold(vec![], async move |curr, act| {
            match act {
                Ok(_) => curr,
                Err(e) => {
                    let mut curr = curr;
                    curr.push(e);
                    curr
                },
            }
        })
        .await;
    
    if user.is_some() { return Ok(user); }
    if errors.len() == 0 { return Ok(None); }
    
    Err(Error::collection(errors))
}
