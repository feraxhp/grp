use hyper::HeaderMap;
use reqwest::Response;

use crate::error::types::ErrorType;
use crate::error::structs::Error;
use crate::common::structs::Context;
use crate::config::Config;
use crate::platform::Platform;
use crate::specific::{gitea, github, gitlab};

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
                return Err(
                    Error::new(
                        ErrorType::Unsupported,
                        vec![name, "Platform variant"]
                    )
                )
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
            Platform::Gitea => { gitea::header::get_auth_header(token) }
            Platform::Gitlab => { gitlab::header::get_auth_header(token) }
        }
    }

    pub(crate) fn get_base_url<S: AsRef<str>>(&self, endpoint: &S) -> String {
        let endpoint = endpoint.as_ref();
        match self {
            Platform::Github => { format!("https://{}", endpoint) }
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { format!("https://{}/api/v1", endpoint) }
            Platform::Gitlab => { format!("https://{}/api/v4", endpoint) }
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
            Platform::Gitea => { gitea::unwrap::unwrap(result, base_message.into(), config, context).await }
            Platform::Gitlab => { gitlab::unwrap::unwrap(result, base_message.into(), config, context).await }
        }
    }
}