use std::process::exit;
use color_print::cprintln;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::girep::config::Config;

#[derive(Serialize, Deserialize)]
struct Error {
    message: String,
    status: String,
}

pub(crate) async fn error_mannager(
    result: Response,
    owner: String,
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
        401 => {
            finish_animation("Bad credentials");
            eprintln!("* Please check your token.");
            eprintln!("  Pconf name: {}", config.pconf.clone());
            eprintln!("  User: {}", owner);
            exit(101);
        },
        404 => {
            finish_animation("User/org does not exist");
            cprintln!("User/org: <m>({})</>", owner);
            exit(101);
        },
        _ => {
            match serde_json::from_str::<Error>(text.as_str()) {
                Ok(error) => {
                    finish_animation(base_message.as_str());
                    eprintln!("{:?}", serde_json::to_string_pretty(&error));
                },
                Err(e) => {
                    finish_animation(base_message.as_str());
                    println!("{:?}", e);
                    cprintln!("<y>Unknown error</> {}", status.as_u16());
                }
            };
            exit(status.as_u16() as i32);
        }
    }
}
