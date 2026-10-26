use serde::{Deserialize, Serialize};
use reqwest::Response;
use mynox::Error;

use crate::helpers::request::{RequestError, unknown};
use crate::common::structs::RequestType;
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
        | (200, RequestType::ListUser)
        | (200, RequestType::ListOrg)
        | (200, RequestType::ListIssues)
        | (200, RequestType::ListIssuesComments)
        | (201, RequestType::CreateRepo { owner: _, repo: _ })
        => return Ok(text),
        (status, _) => {
            let error: ErrorDeserialize = JSON::from_str(&text)?;
            unknown(status, base_message, Some(error.message), request_type)
        }
    };
    
    // let error = match status.as_u16() {
    //     200 => { return Ok(text) }
    //     201 if matches!(context.request_type, RequestType::CreateRepo) => { return Ok(text) },
    //     202 if matches!(context.request_type, RequestType::DeleteOrg) => { return Ok(text) },
    //     401 => Request::unauthorized(
    //         &config.pconf,
    //         context.owner.expect("The owner  must be provided in the context"),
    //         empty_notes!()
    //     ),
    //     403 if matches!(context.request_type, RequestType::DeleteRepo)
    //         => Request::bad_token_scope(&config.pconf, vec!["delete_repo"]),
    //     404 if matches!(context.request_type, RequestType::DeleteRepo)
    //         => NotFound::repository(
    //             &config.pconf, 
    //             context.owner.expect("The owner  must be provided in the context"), 
    //             context.repo.expect("The repo name must be provided in the context")
    //         ),
    //     404 if matches!(
    //         context.request_type,
    //         RequestType::OrgList |
    //         RequestType::DeleteOrg
    //     ) => NotFound::organization(
    //         context.owner.expect("The owner  must be provided in the context"), 
    //         empty_notes!()
    //     ),
    //     422 if matches!(context.request_type, RequestType::CreateRepo)
    //         => AlreadyExist::repository(
    //             &config.pconf, 
    //             context.owner.expect("The owner  must be provided in the context"), 
    //             context.repo.expect("The repo name must be provided in the context")
    //         ),
    //     _ => {
    //         let error: ErrorDeserialize = serde_json::from_str(&text)
    //             .unwrap_or_else(|_| {
    //                 ErrorDeserialize {
    //                     message: text.to_string(),
    //                     status: status.to_string(),
    //                 }
    //             }
    //         );
    //         
    //         Error::new(
    //             "unknown",
    //             base_message,
    //             "Something went wrong:",
    //             vec![
    //                 cformat!("  » Code: <r,i>{}</>", status),
    //                 cformat!("  » Error: {}", error.message)
    //             ],
    //             empty_notes!()
    //         )
    //     }
    // };

    Err(error)
}
