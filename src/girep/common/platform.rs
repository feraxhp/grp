use std::process::exit;
use color_print::cprintln;
use hyper::HeaderMap;
use reqwest::Response;
use crate::girep::error::structs::Error;
use crate::girep::common::structs::Context;
use crate::girep::config::Config;
use crate::girep::platform::Platform;
use crate::girep::specific::{gitea, github, gitlab};

impl Platform {
    pub(crate) fn matches(name: &str) -> Platform {
        let platform: Platform = match name {
            "github" => Platform::Github,
            "gitea" => Platform::Gitea,
            "gitlab" => Platform::Gitlab,
            "codeberg" => Platform::Codeberg,
            name => {
                cprintln!("* Error: <i>{}</> is not a valid platform", name);
                exit(1)
            }
        };

        platform
    }

    pub fn get_auth_header<S: AsRef<str>>(&self, token: &S) -> HeaderMap {
        let token = token.as_ref().to_string();
        match self {
            Platform::Github => { github::header::get_auth_header(token) }
            Platform::Codeberg |
            Platform::Gitea => { gitea::header::get_auth_header(token) }
            Platform::Gitlab => { gitlab::header::get_auth_header(token) }
        }
    }

    pub fn get_base_url<S: AsRef<str>>(&self, endpoint: &S) -> String {
        let endpoint = endpoint.as_ref();
        match self {
            Platform::Github => { format!("https://{}", endpoint) }
            Platform::Codeberg |
            Platform::Gitea => { format!("https://{}/api/v1", endpoint) }
            Platform::Gitlab => { format!("https://{}/api/v4", endpoint) }
        }
    }

    pub async fn unwrap<T: Into<String>>(&self,
        result: Response,
        base_message: T,
        config: &Config,
        context: Context,
    ) -> Result<String, Error> {
        match self {
            Platform::Github => { github::unwrap::unwrap(result, base_message.into(), config, context).await }
            Platform::Codeberg |
            Platform::Gitea => { gitea::unwrap::unwrap(result, base_message.into(), config, context).await }
            Platform::Gitlab => { gitlab::unwrap::unwrap(result, base_message.into(), config, context).await }
        }
    }
}