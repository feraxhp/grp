use mynox::Error;

use crate::common::structs::RequestType;
use crate::platform::Platform;
use crate::config::Config;
use crate::structs::User;


impl Platform {
    /// # Return
    /// a the logged user as `grp_core::structs::User`
    /// 
    /// # Error
    /// a `grp_core::Error` containing the detail of the error. 
    pub async fn get_logged_user(&self, conf: &Config) -> Result<User, Error> {        
        let url = match &self {
            Platform::Github |
            Platform::Gitlab |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { 
               format!("{}/user", self.get_base_url(conf))
            },
        };
        
        let result = self.get(url, true, conf).await?;
        
        let text = self.unwrap(
            result, "Failed during fetch of logged user",
            conf, &RequestType::GetLoggedUser
        ).await?;

        User::from_text(&text, self)
    }
}
