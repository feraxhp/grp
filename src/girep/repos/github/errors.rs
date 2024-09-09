use std::process::exit;
use color_print::cprintln;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::girep::config::Config;
use crate::girep::repos::comond::structs::{DebugData, Rtype};

#[derive(Serialize, Deserialize)]
struct Error {
    message: String,
    status: String,
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
        200 => { text },
        201 if matches!(debug_data.rtype, Rtype::Create) => { text },
        401 => {
            finish_animation("Bad credentials");
            eprintln!("* Please check your token.");
            eprintln!("  Pconf name: {}", config.pconf.clone());
            eprintln!("  User: {}", debug_data.owner);
            exit(101);
        },
        403 if matches!(debug_data.rtype, Rtype::Delete)=> {
            finish_animation("Forbidden");
            eprintln!("* Please check your token.");
            eprintln!("  You must add the following scopes: ");
            cprintln!("    <#e3750e>1. <m>delete_repo</>");
            cprintln!("* Pconf name: {}", config.pconf.clone());
            exit(101);
        },
        404 if matches!(debug_data.rtype, Rtype::Delete) => {
            finish_animation("Repository not found");
            cprintln!("Repository: <m>({}/{})</>", debug_data.owner, debug_data.repo.clone().unwrap());
            exit(101);
        },
        404 => {
            finish_animation("User/org does not exist");
            cprintln!("User/org: <m>({})</>", debug_data.owner);
            if matches!(debug_data.rtype, Rtype::Create) {
                cprintln!("  The user you provide is not an org");
                cprintln!("  Neither is the logged user");

                cprintln!("  <y>+ Please provide a valid org name</>");
            }
            exit(101);
        },
        422 if matches!(debug_data.rtype, Rtype::Create) => {
            finish_animation("Repository already exists");
            cprintln!("Repository: <m>({}/{})</>", debug_data.owner, debug_data.repo.clone().unwrap());
            exit(101);
        },
        _ => {
            match serde_json::from_str::<Error>(text.as_str()) {
                Ok(error) => {
                    finish_animation(base_message.as_str());
                    eprintln!("{}", error.message.clone());
                },
                Err(e) => {
                    finish_animation(base_message.as_str());
                    eprintln!("{:?}", e);
                    cprintln!("<y>Unknown error</> {}", status.as_u16());
                }
            };
            exit(status.as_u16() as i32);
        }
    }
}
