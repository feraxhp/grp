use reqwest::Response;
use mynox::Error;

use crate::helpers::request::{RequestError, unauthorized};
use crate::common::structs::RequestType;
use crate::helpers::request::unknown;
use crate::config::Config;
use crate::JSON;

use super::parser::ErrorDeserialize;


pub async fn unwrap(
    result: Response,
    base_message: String,
    _config: &Config,
    request_type: &RequestType,
) -> Result<String, Error> {

    let status = result.status();
    let text = result.text().await.getting_body()?;
    
    let error = match (status.as_u16(), request_type) {
          (200, RequestType::ListRepo { owner: _ })
        | (200, RequestType::RepositoryDetails { owner: _,  repo: _ })
        | (200, RequestType::GetLoggedUser)
        | (200, RequestType::ListOrg{ way: _ })
        | (200, RequestType::ListIssues)
        | (200, RequestType::ListIssuesComments{ owner: _, repo: _, number: _ })
        | (200, RequestType::ListTodos)
        | (201, RequestType::CreateOrg { name: _ })
        | (201, RequestType::CreateRepo { owner: _, repo: _ })
        => return Ok(text),
        (401, _) => unauthorized(request_type),
        (status, _) => {
            let error: ErrorDeserialize = JSON::from_str(&text)?;
            unknown(status, base_message, error.text, request_type)
        }
    };
    
    Err(error)
}
