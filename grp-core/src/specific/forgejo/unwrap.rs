use serde::{Deserialize, Serialize};
use mynox::Error;

use crate::helpers::already_exist;
use crate::helpers::not_found::not_found;
use crate::helpers::request::{RequestError, bad_token_scope, unauthorized, unknown};
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
        | (200, RequestType::GetLoggedUser)
        | (200, RequestType::ListOrg{ way: _ })
        | (200, RequestType::ListIssues)
        | (200, RequestType::ListIssuesComments{ owner: _, repo: _, number: _ })
        | (201, RequestType::CreateRepo { owner: _, repo: _ })
        | (201, RequestType::CreateOrg { name: _ })
        | (202, RequestType::DeleteOrg { permanent: _, name: _  })
            => return Ok(text),
        (403, RequestType::CreateRepo { 
            owner: _, 
            repo: _,
        }) => bad_token_scope(["write:user"], request_type),
        (404, _) => not_found(&config.pconf, request_type),
        (409, RequestType::CreateRepo { owner, repo  }) => already_exist::repo(&config.pconf, owner, repo),
        (422, RequestType::CreateOrg { name }) => already_exist::organization(name),
        (status, _) => {
            let error: ErrorDeserialize = JSON::from_str(&text)
                .unwrap_or_else(|_| {
                    ErrorDeserialize {
                        message: text.to_string(),
                        status: status.to_string(),
                    }
                })
            ;
            
            match error.message.as_str() {
                "user does not exist [uid: 0, name: ]" => unauthorized(request_type),
                "GetOrgByName" if matches!(request_type, RequestType::CreateRepo { owner: _, repo: _ }) => {
                    not_found(&config.pconf, request_type)
                },
                m if m.starts_with("{\"message\":\"Must be an organization owner\"") => {
                    unauthorized(request_type)
                        .extend_note("you must be an organization owner")
                },
                m if m.starts_with("user redirect does not exist [name: ") => {
                    not_found(&config.pconf, request_type)
                },
                m if m.starts_with("token does not have at least one of required scope(s):")  => {
                    let scopes_index = (error.message.find("[").unwrap() + 1)..error.message.find("]").unwrap();
                    let scopes = error.message[scopes_index].split(",");
                    
                    bad_token_scope(scopes, request_type)
                },
                
                message => unknown(status, base_message, Some(message.to_string()), request_type)
            }
            
            
        }
    };
    
    // let error = match status.as_u16() {
    //     
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
    //             
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
