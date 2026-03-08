use reqwest::Response;
use color_print::cformat;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::errors::already_exist::AlreadyExist;
use crate::error::errors::not_found::NotFound;
use crate::error::errors::request::Request;
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
        200 |
        201 if matches!(context.request_type, RequestType::Create) => { return Ok(text) },
        202 if matches!(context.request_type, RequestType::DeleteOrg) => { return Ok(text) },
        401 => Request::unauthorized(
            &config.pconf,
            &context.owner.expect("The owner  must be provided in the context"),
            empty_notes!()
        ),
        403 if matches!(context.request_type, RequestType::Delete)
            => Request::bad_token_scope(&config.pconf, vec!["delete_repo"]),
        404 if matches!(context.request_type, RequestType::Delete)
            => NotFound::repository(
                &config.pconf, 
                &context.owner.expect("The owner  must be provided in the context"), 
                &context.repo.expect("The repo name must be provided in the context")
            ),
        404 if matches!(
            context.request_type,
            RequestType::ListOrg |
            RequestType::DeleteOrg
        ) => NotFound::organization(
            &context.owner.expect("The owner  must be provided in the context"), 
            empty_notes!()
        ),
        422 if matches!(context.request_type, RequestType::Create)
            => AlreadyExist::repository(
                &config.pconf, 
                &context.owner.expect("The owner  must be provided in the context"), 
                &context.repo.expect("The repo name must be provided in the context")
            ),
        _ => {
            let error: ErrorDeserialize = serde_json::from_str(&text)
                .unwrap_or_else(|_| {
                    ErrorDeserialize {
                        message: format!("{}", text),
                        status: status.to_string(),
                    }
                }
            );
            
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
    };

    Err(error)
}
