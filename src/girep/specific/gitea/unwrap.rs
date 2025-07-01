use crate::girep::config::Config;
use color_print::cformat;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::girep::error::structs::Error;
use crate::girep::error::types::ErrorType;
use crate::girep::common::structs::{Context, RequestType};
use crate::{location, repo};

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
        201 if matches!(context.request_type, RequestType::Create) |
               matches!(context.request_type, RequestType::CreateOrg) 
                => { return Ok(text) },
        403 if matches!(context.request_type, RequestType::Create) => {
            Error::new(
                ErrorType::BadTokenScope, 
                vec![
                    config.pconf.as_str(),
                    "write:user"
                ]
            )
        }
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
        404 if matches!(context.request_type, RequestType::DeleteOrg) => {
            Error::new(
                ErrorType::NotOrganizationFound,
                vec![
                    context.owner
                        .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
                ]
            )
        },
        409 if matches!(context.request_type, RequestType::Create) => {
            let repo = repo!(
                context.owner
                    .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
                context.repo
                    .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?
            );
            Error::new(
                ErrorType::AlreadyExists,
                vec!["Repo".to_string(), repo]
            )
        },
        422 if matches!(context.request_type, RequestType::CreateOrg) => {
            Error::new(
                ErrorType::AlreadyExists,
                vec![
                    "Orgs".to_string(),
                    context.owner
                        .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?
                ]
            )
        },
        status => {
            let error: ErrorDeserialize = serde_json::from_str(&text)
                .unwrap_or_else(|_| {
                    ErrorDeserialize {
                        message: format!("{}", text),
                        status: status.to_string(),
                    }
                }
            );
            
            match error.message.as_str() {
                "user does not exist [uid: 0, name: ]" => {
                    Error::new(
                        ErrorType::Unauthorized,
                        vec![
                            config.pconf.as_str().to_string(),
                            context.owner
                                .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?
                        ]
                    )
                },
                "GetOrgByName" if matches!(context.request_type, RequestType::Create) => {
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
                _ if error.message.starts_with("user redirect does not exist [name: ") => {
                    Error::new(
                        ErrorType::NotOwnerFound,
                        vec![
                            context.owner
                                .ok_or_else(||Error::new(ErrorType::Incomplete,vec![location!()]))?,
                        ]
                    )
                },
                _ if error.message.starts_with("token does not have at least one of required scope(s):") => {

                    let scopes_index = (error.message.find("[").unwrap() + 1)..error.message.find("]").unwrap();
                    let scopes = error.message[scopes_index].split(",");
                    let mut content = vec![config.pconf.as_str()];
                    content.extend(scopes);
                    Error::new(
                        ErrorType::BadTokenScope,
                        content
                    )
                },
                _ => {
                    Error::new_custom(
                        base_message,
                        vec![
                            cformat!("<y>* Something went wrong: </>",),
                            cformat!("  » Code: <r,i>{}</>", status),
                            cformat!("  » Error: {}", error.message)
                        ]
                    )
                }
            }
        }
    };
    
    Err(error)
}
