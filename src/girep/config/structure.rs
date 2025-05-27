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
use color_print::cprintln;
use serde::{Deserialize, Serialize};
use crate::girep::config::loader::load_configurations;
use crate::girep::config::save::save_config;

#[derive(Serialize, Deserialize)]
pub(crate) struct Pconf {
    pub name: String,
    pub owner: String,
    pub token: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Usettings {
    pub default: String,
    #[serde(rename = "pconf")]
    pub pconfs: Vec<Pconf>,
}

impl Pconf {
    pub fn new(name: String, owner: String, token: String, r#type: String, endpoint: String) -> Self {
        Pconf {
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

    pub(crate) fn clone(&self) -> Pconf {
        let temp: Pconf = Pconf::new(
            self.name.clone(),
            self.owner.clone(),
            self.token.clone(),
            self.r#type.clone(),
            self.endpoint.clone(),
        );
        temp
    }
}

impl Usettings {
    pub(crate) fn set_default(&mut self, default: String) {
        self.default = default.clone();
        match save_config(self) {
            Ok(_) => eprintln!("Default set to {}", default),
            Err(e) =>
                eprintln!("Failed to save the default configuration\nError: {:?}", e),
        };
    }

    pub(crate) fn add_config(&mut self, config: Pconf) {
        self.pconfs.push(config);
        save_config(self).unwrap();
    }

    pub(crate) fn get_repos(&self) -> Vec<Pconf> {
        let mut repos: Vec<Pconf> = Vec::new();

        for repo in self.pconfs.iter() {
            repos.push(repo.clone());
        }

        repos
    }

    pub(crate) fn get_pconf(&self, name: String) -> Option<Pconf> {
        for repo in self.pconfs.iter() {
            if repo.name == name {
                return Some(repo.clone());
            }
        }
        None
    }

    pub(crate) fn get_default(&self) -> Pconf {
        match self.get_pconf(self.default.clone()) {
            Some(repo) => repo,
            None => {
                if self.pconfs.is_empty() {
                    eprintln!(" No configurations found");
                    cprintln!(" * Please add a configuration using <bg:#333333, #ffffff>'grp config add'</>");
                    std::process::exit(1);
                }
                let default = self.pconfs[0].clone();
                let mut uc = load_configurations();
                uc.set_default(default.name.clone());

                default
            }
        }
    }

    pub(crate) fn matches_pconf(&self, pconf: Option<&String>) -> Pconf{
        match pconf {
            Some(clist) => {
                match self.get_pconf(clist.clone()) {
                    Some(pconf) => pconf,
                    None => {
                        let pconf = self.get_default();
                        if !clist.eq("-") {
                            eprintln!("{} is not a valid pconf name", clist);
                            eprintln!("using default pconf: {}", pconf.name.clone());
                        }
                        pconf.clone()
                    },
                }
            },
            None => { self.get_default() }
        }
    }
}
