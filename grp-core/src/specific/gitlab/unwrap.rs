use reqwest::Response;
use color_print::cformat;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::error::errors::request::Request;
use crate::error::structs::Error;
use crate::common::structs::{Context, RequestType};
use crate::empty_notes;

#[derive(Serialize, Deserialize)]
struct ErrorDeserialize {
    message: Option<Value>,
    text: Option<String>,
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
        201 if matches!(context.request_type, RequestType::Create) => { return Ok(text) },
        201 if matches!(context.request_type, RequestType::CreateOrg) => { return Ok(text) },
        202 if matches!(context.request_type, RequestType::Create) => { return Ok(text) },
        
        401 => Request::unauthorized(
            &config.pconf,
            &context.owner.expect("The owner  must be provided in the context"),
            empty_notes!()
        ),
        status => {
            let error: ErrorDeserialize = serde_json::from_str(&text)
                .unwrap_or_else(|_| {
                    ErrorDeserialize {
                        message: None,
                        text: Some(format!("{}", text))
                    }
                }
            );
            
            Error::new(
                "unknown",
                base_message,
                "Something went wrong:",
                vec![
                    cformat!("  » Code: <r,i>{}</>", status),
                    cformat!("  » Error: {}", error.text.unwrap_or(cformat!("<r>No error message provided</>")))
                ],
                empty_notes!()
            )
        }
    };
    
    Err(error)
}
