use std::process::exit;
use color_print::cprintln;
use reqwest::Response;
use serde::Deserialize;
use crate::girep::config::Config;

#[derive(Deserialize)]
struct Error {
    message: String,
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
        200 => { return text; },
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
                    eprintln!("  User: {}", owner);
                },
                _ if error.message.starts_with("user redirect does not exist [name: ") => {
                    finish_animation("User/org does not exist");
                    cprintln!("User/org: <m>({})</>", owner);
                },
                _ => {
                    finish_animation(base_message.as_str());
                    println!("{}", &text);
                    cprintln!("<y>Unknown error</> {}", status.as_u16());
                }
            }
        }
    };
    exit(101);
}