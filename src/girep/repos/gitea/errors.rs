use crate::girep::config::Config;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::repos::common::structs::{DebugData, Rtype};
use color_print::cformat;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize)]
struct ErrorDeserialize {
    message: String,
}

pub(crate) async fn error_mannager(
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
            match status.clone().as_u16() {
                200 => { Ok(text) },
                201 if matches!(debug_data.rtype, Rtype::Create) => { Ok(text) },
                404 if matches!(debug_data.rtype, Rtype::Delete) => {
                    Err(
                        Error::new(
                            ErrorType::NotFound,
                            vec![
                                debug_data.owner.as_str(),
                                debug_data.repo.clone().unwrap().as_str(),
                            ]
                        )
                    )
                },
                409 if matches!(debug_data.rtype, Rtype::Create) => {
                    Err(
                        Error::new(
                            ErrorType::AlreadyExists,
                            vec![
                                debug_data.owner.as_str(),
                                debug_data.repo.clone().unwrap().as_str(),
                            ]
                        )
                    )
                },
                _ => {
                    let error: ErrorDeserialize = serde_json::from_str(&text)
                        .unwrap_or_else(|_| {
                            ErrorDeserialize {
                                message: format!("{}", text)
                            }
                        }
                    );

                    match error.message.as_str() {
                        "user does not exist [uid: 0, name: ]" => {
                            Err(
                                Error::new(
                                    ErrorType::Unauthorized,
                                    vec![
                                        config.pconf.as_str(),
                                        debug_data.owner.as_str(),
                                    ]
                                )
                            )
                        },
                        "GetOrgByName" if matches!(debug_data.rtype, Rtype::Create) => {
                            Err(
                                Error::new(
                                    ErrorType::NotFound,
                                    vec![
                                        debug_data.owner.as_str(),
                                        debug_data.repo.clone().unwrap().as_str(),
                                    ]
                                )
                            )
                        },
                        _ if error.message.starts_with("user redirect does not exist [name: ") => {
                            Err(
                                Error::new(
                                    ErrorType::NotFound,
                                    vec![
                                        debug_data.owner.as_str(),
                                    ]
                                )
                            )
                        },
                        _ if error.message.starts_with("token does not have at least one of required scope(s):") => {

                            let scopes_index = (error.message.find("[").unwrap() + 1)..error.message.find("]").unwrap();
                            let scopes = error.message[scopes_index].split(",");
                            let mut content = vec![config.pconf.as_str()];
                            content.extend(scopes);
                            Err(
                                Error::new(
                                    ErrorType::BadTokenScope,
                                    content
                                )
                            )
                        },
                        _ => {
                            Err(
                                Error::new_custom(
                                    base_message,
                                    vec![
                                        cformat!("<y>* Something went wrong: </>",),
                                        cformat!("  <r,i>{}</>", status.as_u16()),
                                        cformat!("  Error: {}", error.message)
                                    ]
                                )
                            )
                        }
                    }
                }
            }
        }
        Err(e) => Err(e)
    }

}