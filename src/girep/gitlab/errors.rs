use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use color_print::cformat;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorDeserialize {
    error: String,
}

pub async fn error_manager(
    result: Response,
    debug_data: DebugData,
    config: Config,
    base_message: String,
) -> Result<String, Error>{

    let status = result.status();
    let text = result.text().await.map_err(|e| {
        Error::new(ErrorType::Unknown, vec![e.to_string().as_str()])
    });

    match text {
        Ok(text) => {
            match status.as_u16() {
                200 => { Ok(text) },
                status => {
                    match serde_json::from_str::<ErrorDeserialize>(text.clone().as_str()) {
                        Ok(error) => {
                            Err(Error::new_custom(
                                base_message,
                                vec![
                                    cformat!("<y>* Something went wrong: </>",),
                                    cformat!("  Status: <m,i>{}</>", status),
                                    cformat!("  Error: <u>{}</>", error.error),
                                ]
                            ))
                        },
                        Err(e) => {

                            Err(
                                Error::new(
                                    ErrorType::Dezerialized,
                                    vec![
                                        format!("{:?}", e).as_str(),
                                        text.as_str()
                                    ]
                                )
                            )
                        }
                    }

                }
            }
        },
        Err(e) => Err(e)
    }
}
