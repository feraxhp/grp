use serde::{Deserialize, Serialize};
use reqwest::Response;
use mynox::Error;

use crate::helpers::request::bad_token_scope;
use crate::helpers::request::RequestError;
use crate::helpers::request::unauthorized;
use crate::helpers::not_found::not_found;
use crate::common::structs::RequestType;
use crate::helpers::request::unknown;
use crate::helpers::already_exist;
use crate::config::Config;
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
            
        (401, _) => unauthorized(request_type),
        (403, RequestType::DeleteRepo { 
            permanent: _, 
            owner: _, 
            repo: _,
        }) => bad_token_scope(["delete_repo"], request_type),
        (404, _) => not_found(&config.pconf, request_type),
        (422, RequestType::CreateOrg { name }) => already_exist::organization(name),
        (422, RequestType::CreateRepo { owner, repo  }) => already_exist::repo(&config.pconf, owner, repo),
        (status, _) => {
            let error: ErrorDeserialize = JSON::from_str(&text)?;
            unknown(status, base_message, Some(error.message), request_type)
        }
    };
    
    Err(error)
}
