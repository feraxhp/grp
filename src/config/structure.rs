/*
 Copyright 2024 feraxhp
 Licensed under the MIT License;

 Configuration structure
 {
	"default": "<repo-name>",
	"repos": [
 		{
	 		"name": "<repo-name>",
			"owner": "<default-owner>",
			"token": "<auth-token>",
			"type": "<repo-type>",
			"endpoint": "<endpoint-to-repo>"
		}
	]
 }
 */
use serde::{Deserialize, Serialize};
use crate::config::loader::load_configurations;
use crate::config::save::save_config;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub name: String,
    pub owner: String,
    pub token: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Root {
    pub default: String,
    pub repos: Vec<Config>,
}

impl Config {
    pub fn new(name: String, owner: String, token: String, r#type: String, endpoint: String) -> Self {
        Config {
            name,
            owner,
            token,
            r#type,
            endpoint,
        }
    }

    pub(crate) fn save(&self) -> Result<(), std::io::Error> {
        let mut root = load_configurations();
        let temp: Config = Config::new(
            self.name.clone(),
            self.owner.clone(),
            self.token.clone(),
            self.r#type.clone(),
            self.endpoint.clone(),
        );

        root.add_config(temp);
        Ok(())
    }
}

impl Root {
    fn set_default(&mut self, default: String) {
        self.default = default;
        save_config(self).unwrap();
    }

    pub(crate) fn add_config(&mut self, config: Config) {
        self.repos.push(config);
        save_config(self).unwrap();
    }


}
