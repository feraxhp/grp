use serde_json::Value;

use crate::common::users::structs::User;
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
    pub fn from_text(text: &String, platform: &Platform) -> Result<Self, Error> {
        let json: Value = JSON::from_str(text)?;
        
        let name: String;
        let id: String;
        let path: Option<String>;
        
        match platform {
            Platform::Github => {
                name = json["login"].as_str().unwrap().to_string();
                id = name.clone();
                path = None
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                name = json["name"].as_str().unwrap().to_string();
                id = name.clone();
                path = None
            },
            Platform::Gitlab => {
                id = json["id"].as_u64().unwrap().to_string();
                name = json["name"].as_str().unwrap().to_string();
                path = Some(json["full_path"].as_str().unwrap().to_string());
            },
        };
        Ok(User { id: id, name: name, path: path })
    }
    
    /// # Return
    /// 
    /// Generates a list of User if the information of the text is a valid list of json 
    /// and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text_array(text: &String, platform: &Platform) -> Result<Vec<Self>, Error> {
        let json: Vec<Value> = JSON::from_str(&text)?;
        
        let orgs: Vec<User> = match platform {
            Platform::Github => json.iter()
                .map(|org| {
                    let name = org["login"].as_str().unwrap().to_string();
                    User { id: name.clone(), name: name.clone(), path: None }
                }).collect(),
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => json.iter()
                .map(|org| {
                    let name = org["name"].as_str().unwrap().to_string();
                    User { id: name.clone(), name: name.clone(), path: None }
                }).collect(),
            Platform::Gitlab => json.iter()
                .map(|org| {
                    let id = org["id"].as_u64().unwrap();
                    let name = org["name"].as_str().unwrap().to_string();
                    let path = org["full_path"].as_str().unwrap().to_string();
                    User { id: id.to_string(), name: name.clone(), path: Some(path) }
                }).collect(),
        };
        
        Ok(orgs)
    }
}

