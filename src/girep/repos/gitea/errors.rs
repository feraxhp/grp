use crate::girep::config::Config;
use color_print::cprintln;
use reqwest::Response;
use serde::Deserialize;
use std::process::exit;
use crate::girep::repos::comond::structs::{DebugData, Rtype};

#[derive(Deserialize)]
struct Error {
    message: String,
}

pub(crate) async fn error_mannager(
    result: Response,
    debug_data: DebugData,
    config: Config,
    base_message: String,
    finish_animation: impl Fn(&str),
) -> String{

    let status = result.status();
    let text = result.text().await.unwrap_or_else(|e| {
        finish_animation(base_message.as_str());
        eprintln!("Failed to read the response text: {}", e);
        cprintln!("<y>Unknown error</>");
        exit(101);
    });

    match status.as_u16() {
        200 if matches!(debug_data.rtype, Rtype::List) => { return text; },
        201 if matches!(debug_data.rtype, Rtype::Create) => { return text; },
        409 if matches!(debug_data.rtype, Rtype::Create) => {
            finish_animation("Repository already exists");
            cprintln!("Repository: <m>({}/{})</>", debug_data.owner, debug_data.repo.clone().unwrap());
        },
        _ => {
            let error: Error = serde_json::from_str(&text)
                .unwrap_or_else(|_| {
                    Error {
                        message: format!("{}", text)
                    }
                }
                );

            match error.message.as_str() {
                "user does not exist [uid: 0, name: ]" => {
                    finish_animation("Bad credentials");
                    eprintln!("* Please check your token.");
                    eprintln!("  Pconf name: {}", config.pconf.clone());
                    eprintln!("  User: {}", debug_data.owner);
                },
                "GetOrgByName" if matches!(debug_data.rtype, Rtype::Create) => {
                    finish_animation("User/org not found");
                    cprintln!("User/org: <m>({})</>", debug_data.owner);
                    cprintln!("  The user you provide is not an org");
                    cprintln!("  Neither is the logged user");

                    cprintln!("  <y>+ Please provide a valid org name</>");
                },
                _ if error.message.starts_with("user redirect does not exist [name: ") => {
                    finish_animation("User/org does not exist");
                    cprintln!("User/org: <m>({})</>", debug_data.owner);
                },
                _ if error.message.starts_with("token does not have at least one of required scope(s):") => {
                    finish_animation("Bad token scope");
                    cprintln!("\n<g>* Pconf name: <s,i>{}</>", config.pconf.clone());
                    cprintln!("<i>* Please check your token.</>");
                    cprintln!("  - You need to add one of the following scopes:");
                    let scopes_index = (error.message.find("[").unwrap() + 1) .. error.message.find("]").unwrap();
                    let scopes = error.message[scopes_index].split(",");
                    for scope in scopes.enumerate() {
                        cprintln!("    <#e3750e>{}. <m>{}</>", scope.0 + 1, scope.1);
                    }
                },
                _ => {
                    finish_animation(base_message.as_str());
                    eprintln!("{}", &text);
                    cprintln!("Request type: <r>{}</>", debug_data.rtype.to_string());
                    cprintln!("<y>Unknown error</> {}", status.as_u16());
                }
            }
        }
    };
    exit(101);
}