use serde::{Deserialize, Serialize};
use mynox::Error;

use crate::helpers::request::{RequestError, unknown};
use crate::common::structs::RequestType;
use crate::config::Config;
use reqwest::Response;
use crate::JSON;


#[derive(Serialize, Deserialize)]
struct ErrorDeserialize {
    message: String,
    status: String,
}

pub async fn unwrap(
    result: Response,
    base_message: String,
    config: &Config,
    request_type: &RequestType, 
) -> Result<String, Error> {
    
    let status = result.status();
    let text = result.text().await.getting_body()?;
    
    let error = match (status.as_u16(), request_type) {
          (200, RequestType::ListRepo { owner: _ })
        | (201, RequestType::CreateRepo { owner: _, repo: _ })
        => return Ok(text),
        (status, _) => {
            let error: ErrorDeserialize = JSON::from_str(&text)?;
            unknown(status, base_message, Some(error.message), request_type)
        }
    };
    
    // let error = match status.as_u16() {
    //     200 => { return Ok(text) },
    //     201 if matches!(context.request_type, RequestType::CreateRepo) |
    //            matches!(context.request_type, RequestType::CreateOrg) 
    //             => { return Ok(text) },
    //     202 if matches!(context.request_type, RequestType::DeleteOrg) => { return Ok(text) },
    //     403 if matches!(context.request_type, RequestType::CreateRepo) 
    //         => Request::bad_token_scope(&config.pconf, vec!["write:user"]),
    //     404 if matches!(context.request_type, RequestType::DeleteRepo) 
    //         => NotFound::repository(
    //             &config.pconf, 
    //             context.owner.expect("The owner  must be provided in the context"), 
    //             context.repo.expect("The repo name must be provided in the context")
    //         ),
    //     404 if matches!(context.request_type, RequestType::DeleteOrg) 
    //         => NotFound::organization(
    //             context.owner.expect("The owner  must be provided in the context"), 
    //             empty_notes!()
    //         ),
    //     409 if matches!(context.request_type, RequestType::CreateRepo) 
    //         => AlreadyExist::repository(
    //             &config.pconf, 
    //             context.owner.expect("The owner  must be provided in the context"), 
    //             context.repo.expect("The repo name must be provided in the context")
    //         ),
    //     422 if matches!(context.request_type, RequestType::CreateOrg) 
    //         => AlreadyExist::organization(
    //             context.owner.expect("The owner  must be provided in the context"), 
    //             empty_notes!()
    //         ),
    //     status => {
    //         let error: ErrorDeserialize = serde_json::from_str(&text)
    //             .unwrap_or_else(|_| {
    //                 ErrorDeserialize {
    //                     message: text.to_string(),
    //                     status: status.to_string(),
    //                 }
    //             }
    //         );
    //         
    //         match error.message.as_str() {
    //             "user does not exist [uid: 0, name: ]" 
    //                 => Request::unauthorized(
    //                     &config.pconf,
    //                     context.owner.expect("The owner  must be provided in the context"),
    //                     empty_notes!()
    //                 ),
    //             "GetOrgByName" if matches!(context.request_type, RequestType::CreateRepo) 
    //                 => NotFound::repository(
    //                     &config.pconf, 
    //                    context.owner.expect("The owner  must be provided in the context"), 
    //                     context.repo.expect("The repo name must be provided in the context")
    //                 ),
    //             _ if 
    //                 error.message.starts_with("{\"message\":\"Must be an organization owner\"") && 
    //                 matches!(context.request_type, RequestType::DeleteOrg) 
    //                 => Request::unauthorized(
    //                     &config.pconf, 
    //                     context.owner.expect("The owner must be provided in the context"),
    //                     vec!["The token must be an organization owner"]
    //                 ),
    //             _ if error.message.starts_with("user redirect does not exist [name: ") 
    //                 => NotFound::user(
    //                     context.owner.expect("The owner  must be provided in the context"), 
    //                     empty_notes!()
    //                 ),
    //             _ if error.message.starts_with("token does not have at least one of required scope(s):") 
    //                 => {
    //                     let scopes_index = (error.message.find("[").unwrap() + 1)..error.message.find("]").unwrap();
    //                     let scopes = error.message[scopes_index].split(",");
    //                     
    //                     Request::bad_token_scope(
    //                         &config.pconf, 
    //                         scopes.collect()
    //                     )
    //                 }
    //             _ => {
    //                 Error::new(
    //                     "unknown",
    //                     base_message,
    //                     "Something went wrong:",
    //                     vec![
    //                         cformat!("  » Code: <r,i>{}</>", status),
    //                         cformat!("  » Error: {}", error.message)
    //                     ],
    //                     empty_notes!()
    //                 )
    //             }
    //         }
    //     }
    // };
    // 
    Err(error)
}
