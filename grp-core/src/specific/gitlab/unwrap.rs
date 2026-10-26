use reqwest::Response;
use mynox::Error;

use crate::helpers::request::RequestError;
use crate::common::structs::RequestType;
use crate::helpers::request::unknown;
use crate::config::Config;
use crate::JSON;

use super::parser::ErrorDeserialize;


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
            unknown(status, base_message, error.text, request_type)
        }
    };
    
    // let error = match status.as_u16() {
    //     200 => { return Ok(text) },
    //     201 if matches!(context.request_type, RequestType::CreateRepo) => { return Ok(text) },
    //     201 if matches!(context.request_type, RequestType::CreateOrg) => { return Ok(text) },
    //     202 if matches!(context.request_type, RequestType::CreateRepo) => { return Ok(text) },
    //     
    //     401 => Request::unauthorized(
    //         &config.pconf,
    //         context.owner.expect("The owner  must be provided in the context"),
    //         empty_notes!()
    //     ),
    //     status => {
    //         let error: ErrorDeserialize = serde_json::from_str(&text)
    //             .unwrap_or_else(|_| {
    //                 ErrorDeserialize {
    //                     message: None,
    //                     text: Some(text.to_string())
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
    //                 cformat!("  » Error: {}", error.text.unwrap_or(cformat!("<r>No error message provided</>")))
    //             ],
    //             empty_notes!()
    //         )
    //     }
    // };
    
    Err(error)
}
