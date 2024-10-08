use crate::girep::config::Config;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use color_print::cformat;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize)]
struct ErrorDeserialize {
    message: String,
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
            match (status.clone().as_u16(), debug_data.rtype.clone()) {
                (200, _) => { Ok(text) },
                (201, Rtype::Create | Rtype::CreateOrg) => { Ok(text) },
                (404, Rtype::Delete) => {
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
                (404, Rtype::DeleteOrg) => {
                    Err(
                        Error::new_custom(
                            ErrorType::NotFound.get_message(),
                            vec![
                                cformat!("* Organization <y,i>{}</> does not exist", debug_data.owner.clone())
                            ]
                        )
                    )
                }
                (409, Rtype::Create) => {
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
                (422, Rtype::CreateOrg) => {
                    Err(
                        Error::new_custom(
                            ErrorType::AlreadyExists.get_message(),
                            vec![
                                cformat!("* The organization name is <i,b>invalid</>"),
                                "  There is another user/org this this name".to_string(),
                                cformat!("  - Name: <m,i>{}</>", debug_data.owner)
                            ]
                        )
                    )
                }
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
                                        cformat!("  Code: <r,i>{}</>", status.as_u16()),
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