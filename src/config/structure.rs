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
        root.add_config(self.clone());
        Ok(())
    }

    pub(crate) fn clone(&self) -> Config {
        let temp: Config = Config::new(
            self.name.clone(),
            self.owner.clone(),
            self.token.clone(),
            self.r#type.clone(),
            self.endpoint.clone(),
        );
        temp
    }
}

impl Root {
    pub(crate) fn set_default(&mut self, default: String) {
        self.default = default.clone();
        match save_config(self) {
            Ok(_) => eprintln!("Default set to {}", default),
            Err(e) =>
                eprintln!("Failed to save the default configuration\nError: {:?}", e),
        };
    }

    pub(crate) fn add_config(&mut self, config: Config) {
        self.repos.push(config);
        save_config(self).unwrap();
    }

    pub(crate) fn get_repos(&self) -> Vec<Config> {
        let mut repos: Vec<Config> = Vec::new();

        for repo in self.repos.iter() {
            repos.push(repo.clone());
        }

        repos
    }
}
