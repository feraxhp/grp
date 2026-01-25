use futures::future::join_all;
use serde_json::Value;

use crate::json::JSON;
use crate::{config::Config, platform::Platform};
use crate::common::users::structs::User;
use crate::common::structs::{Context, RequestType};
use crate::common::pagination::pagination;
use crate::error::structs::Error;


pub async fn by_full_path(platform: &Platform, name: &String, conf: &Config) -> Result<Option<User>, Error> {
    let url = match platform {
        Platform::Gitlab => {
            format!("{}/groups?search={}", platform.get_base_url(&conf.endpoint), name)
        },
        _ => unimplemented!("Gitea has no actions for this")
    };
    
    let mut errors: Vec<Error> = Vec::new();
    let mut headers = platform.get_auth_header(&conf.token);
    headers.remove("authorization");
    let (responses, error) = pagination(url, headers).await;
    if let Some(e) = error { errors.push(e); }
    
    let responses: Vec<_> = responses.into_iter()
        .map(|response| {
            let context = Context {
                request_type: RequestType::List,
                owner: None, repo: None, additional: None,
            };
            
            platform.unwrap(
                response, "Failed to fetch organizations",
                &conf, context
            )
        }).collect();
    
    let responses = join_all(responses).await;
    
    let (responses, response_errors): (Vec<_>, Vec<_>) = responses.into_iter().partition(Result::is_ok);
    errors.extend(response_errors.into_iter().map(Result::unwrap_err));
    
    for response in responses {
        match response {
            Ok(text) => {
                let json: Vec<Value> = JSON::from_str(&text)?;
                
                if json.is_empty() { return Ok(None); }
                
                match json.iter().find(|u| u["full_path"].as_str().unwrap() == name) {
                    Some(user) => {
                        let user = User {
                            id: user["id"].as_u64().unwrap().to_string(),
                            name: user["name"].as_str().unwrap().to_string(),
                            path: Some(user["full_path"].as_str().unwrap().to_string()),
                        };
                        return Ok(Some(user));
                    },
                    None => continue,
                }
            }
            Err(e) => errors.push(e),
        }
    };
    
    if !errors.is_empty() { return Err(Error::colection(errors)); }
    Ok(None)
}