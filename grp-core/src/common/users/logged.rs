use crate::error::structs::Error;
use crate::common::structs::{Context, RequestType};
use crate::structs::User;
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

        User::from_text(&text, self)
    }
}
