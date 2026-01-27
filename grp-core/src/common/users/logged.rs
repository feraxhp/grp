use serde_json::Value;

use crate::error::structs::Error;
use crate::common::structs::{Context, RequestType};
use crate::common::users::structs::User;
use crate::config::Config;
use crate::platform::Platform;

impl Platform {
    /// # Return
    /// a the logged user as `grp_core::structs::User`
    /// 
    /// # Error
    /// a `grp_core::Error` containing the detail of the error. 
    pub async fn get_logged_user(&self, conf: &Config) -> Result<User, Error> {
        let context = Context {
            request_type: RequestType::UserList,
            owner: Some(conf.user.clone()),
            repo: None,
            additional: None,
        };
        
        let url = match &self {
            Platform::Github |
            Platform::Gitlab |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { 
               format!("{}/user", self.get_base_url(&conf.endpoint))
            },
        };
        
        let result = self.get(url, true, conf).await?;
        
        let text = self.unwrap(
            result, "Failed during fetch of logged user",
            conf, context
        ).await?;

        let json: Value = serde_json::from_str(&text).map_err(Error::from_serde(&text))?;

        let user = match &self {
            Platform::Github =>  {
                let name = json["login"].as_str().unwrap().to_string();
                User { id: name.clone(), name: name.clone(), path: None }
            },
            Platform::Gitea |
            Platform::Forgejo |
            Platform::Codeberg => {
                let name = json["login"].as_str().unwrap().to_string();
                User { id: name.clone(), name: name.clone(), path: None }
            },
            Platform::Gitlab => {
                let id = json["id"].as_u64().unwrap().to_string();
                let name = json["username"].as_str().unwrap().to_string();
                User { id: id.clone(), name: name.clone(), path: None }
            },
        };
        
        return Ok(user);
    }
}
