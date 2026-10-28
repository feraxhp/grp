use futures::StreamExt;
use std::future;

use mynox::Error;

use crate::specific::gitlab::parser::Group;
use crate::common::traits::Convert;
use crate::structs::RequestType;
use crate::platform::Platform;
use crate::config::Config;
use crate::structs::User;
use crate::json::JSON;


pub async fn by_full_path(name: &String, config: &Config) -> Result<Option<User>, Error> {
    let platform = Platform::Gitlab;
    let url = format!("{}/groups?search={}", platform.get_base_url(config), name);
    
    let mut user: Option<User> = None;
    
    let errors: Vec<Error> = platform.pagginate(url, config, RequestType::ListOrg { way: "gitlab search".to_string() }, 1)
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
    if errors.is_empty() { return Ok(None); }
    
    todo!();
    // Err(errors.into_iter().collect::<OptionError>().into_option().unwrap();)
}
