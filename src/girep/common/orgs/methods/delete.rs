use crate::girep::animations;
use crate::girep::animations::animation::Animation;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::config::config::Config;
use crate::girep::platform::Platform;

impl Platform {
    pub(crate) async fn delete_org(&self, name: String, config: Config) -> Result<(), Error> {
        let header_map = self.get_auth_header(config.token.clone());

        let url = self.url_delete_org(config.endpoint.clone(), name.clone());

        let load_animation = animations::delition::Delete::new("Deleting organization ...");

        let client = reqwest::Client::new();
        let result = match client.delete(url)
            .headers(self.get_auth_header(config.token.clone())).send().await {
            Ok(response) => response,
            Err(e) => {
                load_animation.finish_with_error("Failed to contact the platform");
                return Err(
                    Error::new(
                        ErrorType::Unknown,
                        vec![
                            e.to_string().as_str(),
                        ]
                    )
                )
            }
        };

        if result.status().as_u16() == 204 {
            load_animation.finish_with_success("Organization deleted successfully!");
            return Ok(());
        }

        let error = self.error_manager(
            result,
            DebugData{
                rtype: Rtype::DeleteOrg,
                owner: name.clone(),
                repo: None,
            },
            config.clone(),
            "Failed to delete organization".to_string(),
        ).await.unwrap_err();

        load_animation.finish_with_error(error.message.as_str());
        Err(error)
    }
}