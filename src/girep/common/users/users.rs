use serde_json::Value;

use crate::girep::common::users::structs::User;
use crate::girep::error::structs::Error;
use crate::girep::platform::Platform;
use crate::girep::json::JSON;


impl User {
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
    
    pub fn from_text_array(text: &String, platform: &Platform) -> Result<Vec<Self>, Error> {
        let json: Vec<Value> = JSON::from_str(&text)?;
        
        let orgs: Vec<User> = match platform {
            Platform::Github => json.iter()
                .map(|org| {
                    let name = org["login"].as_str().unwrap().to_string();
                    User { id: name.clone(), name: name.clone(), path: None }
                }).collect(),
            Platform::Codeberg |
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

