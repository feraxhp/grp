use std::future;
use futures::StreamExt;

use crate::common::traits::Convert;
use crate::json::JSON;
use crate::specific::gitlab::parser::{Group};
use crate::structs::{Context, RequestType};
use crate::{config::Config, platform::Platform};
use crate::structs::User;
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
    
    let errors: Vec<Error> = platform.pagginate(url, config, context, 1)
        .map(|result| -> Result<Vec<Group>, Error>{
            match result {
                Ok(s) => JSON::from_str(&s.result),
                Err(e) => Err(e),
            }
        })
        .take_while(|s| {
            let groups = match s {
                Ok(s) => s,
                Err(_) => return future::ready(false),
            };
            
            if groups.is_empty() { return future::ready(true) }
            
            match groups.iter().find(|group| group.full_path.as_str() == name) {
                Some(group) => {
                    user = Some(group.convert());
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
