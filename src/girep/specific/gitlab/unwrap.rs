use crate::girep::config::Config;
use color_print::cformat;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use crate::girep::error::structs::Error;
use crate::girep::error::types::ErrorType;
use crate::girep::common::structs::{Context, RequestType/*, RequestType*/};
use crate::{location/*, repo*/};

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
    let text = result.text().await.map_err(|e| {
        Error::new(ErrorType::Unknown, vec![e.to_string().as_str()])
    })?;

    let error = match status.as_u16() {
        200 => { return Ok(text) },
        201 if matches!(context.request_type, RequestType::Create) => { return Ok(text) },
        201 if matches!(context.request_type, RequestType::CreateOrg) => { return Ok(text) },
        202 if matches!(context.request_type, RequestType::Create) => { return Ok(text) }
        401 =>  Error::new(
            ErrorType::Unauthorized,
            vec![
                config.pconf.clone(),
                context.owner.ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
            ]
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
            
            match error.message {
                Some(message) => {
                    Error::new_custom(
                        base_message,
                        vec![
                            cformat!("<y>* Something went wrong: </>",),
                            cformat!("  » Code: <r,i>{}</>", status),
                            match to_string_pretty(&message) {
                                Ok(msg) => cformat!("  » Error: <b>{}</>", msg),
                                Err(_) => cformat!("  » Error: <b>{:#?}</>", message)
                            },
                        ]
                    )
                },
                None => {
                    Error::new_custom(
                        base_message,
                        vec![
                            cformat!("<y>* Something went wrong: </>",),
                            cformat!("  » Code: <r,i>{}</>", status),
                            cformat!("  » Error: <b>{}</>", error.text.unwrap_or_else(|| "No error message provided".to_string()))
                        ]
                    )
                }
            }
        }
    };
    
    Err(error)
}
