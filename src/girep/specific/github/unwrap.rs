use crate::girep::config::Config;
use color_print::cformat;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::girep::error::structs::Error;
use crate::girep::error::types::ErrorType;
use crate::girep::common::structs::{Context, RequestType};
use crate::location;

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
    let text = result.text().await.map_err(|e| {
        Error::new(ErrorType::Unknown, vec![e.to_string().as_str()])
    })?;


    let error = match status.as_u16() {
        200 => { return Ok(text) },
        201 if matches!(context.request_type, RequestType::Create) => { return Ok(text) },
        401 => Error::new(
            ErrorType::Unauthorized,
            vec![
                config.pconf.clone(),
                context.owner.ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
            ]
        ),
        403 if matches!(context.request_type, RequestType::Delete) => {
            Error::new(
                ErrorType::BadTokenScope,
                vec![
                    config.pconf.as_str(),
                    "delete_repo"
                ]
            )
        },
        404 if matches!(context.request_type, RequestType::Delete) => {
            Error::new(
                ErrorType::NotRepoFound,
                vec![
                    context.owner
                        .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
                    context.repo
                        .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
                ]
            )
        },
        404 => {
            let mut content = vec![
                context
                    .owner.ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
            ];

            if matches!(context.request_type, RequestType::Create) {
                let message = cformat!(
                    "  The user you provide is not an org\
                      Neither is the logged user\
                    <y>+ Please provide a valid org name"
                );
                content.push(message);
            }

            Error::new(ErrorType::NotOwnerFound, content)
        },
        422 if matches!(context.request_type, RequestType::Create) => {
            Error::new(
                ErrorType::AlreadyExists,
                vec![
                    "Repo".to_string(),
                    context.owner
                        .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
                    context.repo
                        .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?
                ]
            )
        },
        _ => {
            match serde_json::from_str::<ErrorDeserialize>(text.clone().as_str()) {
                Ok(error) => {
                        Error::new_custom(
                            base_message,
                            vec![
                                cformat!("<y>* Something went wrong: </>",),
                                cformat!("  <r,i>{}</>", error.status),
                                cformat!("  Error: {}", error.message)
                            ]

                    )
                },
                Err(e) => {
                        Error::new(
                            ErrorType::Unknown,
                            vec![
                                format!("{:?}", e).as_str(),
                                text.as_str()
                            ]
                        )

                }
            }
        }
    };

    Err(error)
}
