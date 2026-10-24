use std::fmt::Display;

use hyper::HeaderMap;
use reqwest::Response;

use crate::empty_notes;
use crate::error::errors::request::Request;
use crate::error::structs::Error;
use crate::common::structs::Context;
use crate::config::Config;
use crate::platform::Platform;
use crate::specific::{forgejo, github, gitlab};

impl Platform {
    /// # Return
    /// An _enum variant_ that matches the given &str.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::Unsupported`.
    pub fn matches(name: &str) -> Result<Platform, Error> {
        let platform: Platform = match name {
            "github" => Platform::Github,
            "gitea" => Platform::Gitea,
            "gitlab" => Platform::Gitlab,
            "codeberg" => Platform::Codeberg,
            "forgejo" => Platform::Forgejo,
            name => {
                return Err(Request::unsuported(name, "Platform variant", empty_notes!()));
            }
        };

        Ok(platform)
    }

    pub(crate) fn get_auth_header<S: AsRef<str>>(&self, token: &S) -> HeaderMap {
        let token = token.as_ref().to_string();
        match self {
            Platform::Github => { github::header::get_auth_header(token) }
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { forgejo::header::get_auth_header(token) }
            Platform::Gitlab => { gitlab::header::get_auth_header(token) }
        }
    }

    pub(crate) fn get_base_url(&self, config: &Config) -> String {
        let endpoint = &config.endpoint;
        let https = if config.unsafed { "http" } else { "https" };
        match self {
            Platform::Github => { format!("{https}://{}", endpoint) }
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { format!("{https}://{}/api/v1", endpoint) }
            Platform::Gitlab => { format!("{https}://{}/api/v4", endpoint) }
        }
    }

    /// this function allows to discern if the platform fails, or success 
    /// procesing the request.
    /// 
    /// # Return
    /// a `String` with the response of the platform only if the response was succesfull.
    /// 
    /// # Error
    /// a `grp_core::Error` containing the detail of the error. 
    pub async fn unwrap<T: Into<String>>(&self,
        result: Response,
        base_message: T,
        config: &Config,
        context: Context,
    ) -> Result<String, Error> {
        match self {
            Platform::Github => { github::unwrap::unwrap(result, base_message.into(), config, context).await }
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { forgejo::unwrap::unwrap(result, base_message.into(), config, context).await }
            Platform::Gitlab => { gitlab::unwrap::unwrap(result, base_message.into(), config, context).await }
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Github => write!(f, "{}", "github"),
            Platform::Gitea => write!(f, "{}", "gitea"),
            Platform::Gitlab => write!(f, "{}", "gitlab"),
            Platform::Codeberg => write!(f, "{}", "codeberg"),
            Platform::Forgejo => write!(f, "{}", "forgejo"),
        }
    }
}
