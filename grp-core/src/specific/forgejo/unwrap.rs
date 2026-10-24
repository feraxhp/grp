use crate::config::Config;
use crate::error::errors::already_exist::AlreadyExist;
use crate::error::errors::not_found::NotFound;
use crate::error::errors::request::Request;
use color_print::cformat;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::error::structs::Error;
use crate::common::structs::{Context, RequestType};
use crate::empty_notes;


#[derive(Serialize, Deserialize)]
struct ErrorDeserialize {
    message: String,
    status: String,
}

pub async fn unwrap(
    result: Response,
    base_message: String,
    config: &Config,
    context: Context,
) -> Result<String, Error> {

    let status = result.status();
    let text = result.text().await.map_err(Request::getting_body)?;

    let error = match status.as_u16() {
        200 => { return Ok(text) },
        201 if matches!(context.request_type, RequestType::Create) |
               matches!(context.request_type, RequestType::CreateOrg) 
                => { return Ok(text) },
        202 if matches!(context.request_type, RequestType::DeleteOrg) => { return Ok(text) },
        403 if matches!(context.request_type, RequestType::Create) 
            => Request::bad_token_scope(&config.pconf, vec!["write:user"]),
        404 if matches!(context.request_type, RequestType::Delete) 
            => NotFound::repository(
                &config.pconf, 
                &context.owner.expect("The owner  must be provided in the context"), 
                &context.repo.expect("The repo name must be provided in the context")
            ),
        404 if matches!(context.request_type, RequestType::DeleteOrg) 
            => NotFound::organization(
                &context.owner.expect("The owner  must be provided in the context"), 
                empty_notes!()
            ),
        409 if matches!(context.request_type, RequestType::Create) 
            => AlreadyExist::repository(
                &config.pconf, 
                &context.owner.expect("The owner  must be provided in the context"), 
                &context.repo.expect("The repo name must be provided in the context")
            ),
        422 if matches!(context.request_type, RequestType::CreateOrg) 
            => AlreadyExist::organization(
                &context.owner.expect("The owner  must be provided in the context"), 
                empty_notes!()
            ),
        status => {
            let error: ErrorDeserialize = serde_json::from_str(&text)
                .unwrap_or_else(|_| {
                    ErrorDeserialize {
                        message: format!("{}", text),
                        status: status.to_string(),
                    }
                }
            );
            
            match error.message.as_str() {
                "user does not exist [uid: 0, name: ]" 
                    => Request::unauthorized(
                        &config.pconf,
                        &context.owner.expect("The owner  must be provided in the context"),
                        empty_notes!()
                    ),
                "GetOrgByName" if matches!(context.request_type, RequestType::Create) 
                    => NotFound::repository(
                        &config.pconf, 
                        &context.owner.expect("The owner  must be provided in the context"), 
                        &context.repo.expect("The repo name must be provided in the context")
                    ),
                _ if 
                    error.message.starts_with("{\"message\":\"Must be an organization owner\"") && 
                    matches!(context.request_type, RequestType::DeleteOrg) 
                    => Request::unauthorized(
                        &config.pconf, 
                        &context.owner.expect("The owner must be provided in the context"),
                        vec!["The token must be an organization owner"]
                    ),
                _ if error.message.starts_with("user redirect does not exist [name: ") 
                    => NotFound::user(
                        &context.owner.expect("The owner  must be provided in the context"), 
                        empty_notes!()
                    ),
                _ if error.message.starts_with("token does not have at least one of required scope(s):") 
                    => {
                        let scopes_index = (error.message.find("[").unwrap() + 1)..error.message.find("]").unwrap();
                        let scopes = error.message[scopes_index].split(",");
                        
                        Request::bad_token_scope(
                            &config.pconf, 
                            scopes.collect()
                        )
                    }
                _ => {
                    Error::new(
                        "unknown",
                        base_message,
                        "Something went wrong:",
                        vec![
                            cformat!("  » Code: <r,i>{}</>", status),
                            cformat!("  » Error: {}", error.message)
                        ],
                        empty_notes!()
                    )
                }
            }
        }
    };
    
    Err(error)
}
