use color_print::cformat;
use serde::Deserialize;
use crate::girep::config::Config;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::repos::common::structs::{DebugData, Rtype};
use crate::girep::repos::common::supported::Platform;

pub(crate) enum UserType {
    Logged, // User that is logged in
    Organization, // Organization that bellongs to the logged user
    Free, // User that is not logged in
}

#[derive(Deserialize)]
struct Transpiler {
    login: String,
}

impl Platform {
    pub async fn get_user_type(&self, name: &str, conf: Config) -> Result<UserType, Error> {
        match self.is_logged_user(name, conf.clone()).await {
            Ok(true) => Ok(UserType::Logged),
            Ok(false) => {
                match self.is_organization(name, conf).await {
                    Ok(true) => Ok(UserType::Organization),
                    Ok(false) => Ok(UserType::Free),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }


    pub(crate) async fn is_logged_user(&self, name: &str, conf: Config) -> Result<bool, Error> {
        let client = reqwest::Client::new();

        let result = client
            .get(format!("{}/user", self.get_base_url(conf.endpoint.clone())))
            .headers(self.get_auth_header(conf.token.clone()))
            .send()
            .await
            .map_err(
                |e| {
                    Error::new(
                        ErrorType::Unknown,
                        vec![
                            cformat!("Failed while fetching user type").as_str(),
                            e.to_string().as_str()
                        ]
                    )
                }
            )?;

        let text = self.error_mannager(
            result,
            DebugData {
                rtype: Rtype::UserList,
                owner: name.to_string(),
                repo: None,
            },
            conf.clone(),
            "Failed while fetching user type".to_string()
        ).await?;

        let transpiler: Transpiler = serde_json::from_str(&text.clone())
            .map_err(
                |e|
                    Error::new(
                        ErrorType::Dezerialized,
                        vec![
                            e.to_string().as_str(),
                            text.as_str()
                        ]
                    )
            )?;
        Ok(transpiler.login == name)

    }

    pub(crate) async fn is_organization(&self, name: &str, conf: Config) -> Result<bool, Error> {
        let client = reqwest::Client::new();

        let result = client
            .get(format!("{}/user/orgs", self.get_base_url(conf.endpoint.clone())))
            .headers(self.get_auth_header(conf.token.clone()))
            .send()
            .await
            .map_err(
                |e| {
                    Error::new(
                        ErrorType::Unknown,
                        vec![
                            cformat!("Failed while fetching user type").as_str(),
                            e.to_string().as_str()
                        ]
                    )
                }
            )?;

        let text = self.error_mannager(
            result,
            DebugData {
                rtype: Rtype::UserList,
                owner: name.to_string(),
                repo: None,
            },
            conf.clone(),
            "Failed while fetching user type".to_string()
        ).await?;

        let transpilers: Vec<Transpiler> = serde_json::from_str(&text.clone())
            .map_err(
                |e|
                    Error::new(
                        ErrorType::Dezerialized,
                        vec![
                            e.to_string().as_str(),
                            text.as_str()
                        ]
                    )
            )?;

        Ok(transpilers.iter().any(|t| t.login == name))
    }
}