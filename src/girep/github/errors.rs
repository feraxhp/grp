use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use color_print::cformat;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorDeserialize {
    message: String,
    status: String,
}

pub async fn error_mannager(
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
                201 if matches!(debug_data.rtype, Rtype::Create) => { Ok(text) },
                401 => {
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
                403 if matches!(debug_data.rtype, Rtype::Delete) => {
                    Err(
                        Error::new(
                            ErrorType::BadTokenScope,
                            vec![
                                config.pconf.as_str(),
                                "delete_repo",
                            ]
                        )
                    )

                },
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
                404 => {
                    let mut content = vec![debug_data.owner.clone()];

                    if matches!(debug_data.rtype, Rtype::Create) {
                        let message = cformat!(
                            "  The user you provide is not an org\
                              Neither is the logged user\
                            <y>+ Please provide a valid org name"
                        );
                        content.push(message);
                    }

                    Err(
                        Error::new(
                            ErrorType::NotFound,
                            content.iter().map(|s| s.as_str()).collect()
                        )
                    )
                },
                422 if matches!(debug_data.rtype, Rtype::Create) => {
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
                    match serde_json::from_str::<ErrorDeserialize>(text.clone().as_str()) {
                        Ok(error) => {
                            Err(
                                Error::new_custom(
                                    base_message,
                                    vec![
                                        cformat!("<y>* Something went wrong: </>",),
                                        cformat!("  <r,i>{}</>", error.status),
                                        cformat!("  Error: {}", error.message)
                                    ]
                                )
                            )
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
