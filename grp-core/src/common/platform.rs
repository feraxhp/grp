use std::fmt::Display;
use std::iter::once;

use reqwest::Response;
use hyper::HeaderMap;

use mynox::Error;
use mynox::error;

use crate::specific::{forgejo, github, gitlab};
use crate::{ErrorType, SUPPORTED_REPOS};
use crate::structs::RequestType;
use crate::platform::Platform;
use crate::config::Config;

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
                return Err(error!(
                    ErrorType::unsuported::variant,
                    "Unsuported platform",
                    "The given platform is not supoted",
                    => format!("Given platform: -{}-", name),
                ).extend_notes_with(|| {
                    once("Suported repos: ".to_string()).chain(
                        SUPPORTED_REPOS.iter().map(|s| {
                            format!("  - {}", s.2)
                        })
                    )
                }));
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
        request_type: &RequestType,
    ) -> Result<String, Error> {
        match self {
            Platform::Github => { github::unwrap::unwrap(result, base_message.into(), config, request_type).await }
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { forgejo::unwrap::unwrap(result, base_message.into(), config, request_type).await }
            Platform::Gitlab => { gitlab::unwrap::unwrap(result, base_message.into(), config, request_type).await }
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Github => write!(f, "github"),
            Platform::Gitea => write!(f, "gitea"),
            Platform::Gitlab => write!(f, "gitlab"),
            Platform::Codeberg => write!(f, "codeberg"),
            Platform::Forgejo => write!(f, "forgejo"),
        }
    }
}
