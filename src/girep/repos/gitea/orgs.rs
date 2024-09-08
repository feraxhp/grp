use crate::girep::base::Platform;
use crate::girep::config::Config;
use crate::girep::repos::gitea::implementation::Gitea;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use color_print::cformat;

#[derive(Deserialize)]
struct Transpiler {
    login: String,
}

fn handle_error(message: &str) -> Result<bool, Error> {
    Err(Error::new(ErrorKind::Other, message.to_string()))
}

pub(crate) async fn is_logged_user(name: &str, conf: Config) -> Result<bool, Error> {
    let client = reqwest::Client::new();

    let result = client
        .get(format!("https://{}/api/v1/user", conf.endpoint))
        .headers(Gitea::get_auth_header(conf.token.clone()))
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    match result.status().as_u16() {
        200 => {
            let transpiler: Transpiler = result
                .json().await
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            Ok(transpiler.login == name)
        },
        401 => handle_error(cformat!("<r>Unauthorized</>").as_str()),
        _ => handle_error(cformat!("<r>Failed while fetching user type</>").as_str())
    }
}