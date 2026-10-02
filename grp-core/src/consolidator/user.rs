use crate::common::traits::Convert;
use crate::specific::{gitea, github, gitlab};
use crate::structs::User;
use crate::error::structs::Error;
use crate::platform::Platform;
use crate::json::JSON;


impl User {
    /// # Return
    /// Generates an instance of a user if the information of 
    /// the text is a valid json and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text<S: AsRef<str>,>(text: &S, platform: &Platform) -> Result<Self, Error> {
        let user: Self = match platform {
            Platform::Github => {
                let user: github::parser::User = JSON::from_str(text)?;
                user.convert()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let user: gitea::parser::User = JSON::from_str(text)?;
                user.convert()
            },
            Platform::Gitlab => {
                let user: gitlab::parser::User = JSON::from_str(text)?;
                user.convert()
            },
        };
        
        Ok(user)
    }
    
    /// # Return
    /// 
    /// Generates a list of User if the information of the text is a valid list of json 
    /// and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text_array<S: AsRef<str>>(text: &S, platform: &Platform) -> Result<Vec<Self>, Error> {
        let users: Vec<Self> = match platform {
            Platform::Github => {
                let users: Vec<github::parser::User> = JSON::from_str(text)?;
                users.into_iter().map(|u| u.convert()).collect()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let users: Vec<gitea::parser::User> = JSON::from_str(text)?;
                users.into_iter().map(|u| u.convert()).collect()
            },
            Platform::Gitlab => {
                let users: Vec<gitlab::parser::User> = JSON::from_str(text)?;
                users.into_iter().map(|u| u.convert()).collect()
            },
        };
        
        Ok(users)
    }
}
