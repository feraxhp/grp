use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pconf {
    pub name: String,
    pub owner: String,
    pub token: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub endpoint: String,
    #[serde(default)]
    pub encripted: bool,
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct RawPconf {
    pub name: String,
    pub owner: String,
    pub r#type: String,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usettings {
    pub default: String,
    #[serde(default = "default_true")] // Keep compatibility to old settings
    pub keyring: bool,
    #[serde(default)] // Keep compatibility to old settings
    pub hidepass: bool,
    #[serde(rename = "pconf")]
    pub pconfs: Vec<Pconf>,
}

fn default_true() -> bool { true }

impl Pconf {
    pub(crate) fn as_raw(&self) -> RawPconf {
        RawPconf { 
            name: self.name.to_owned(),
            owner: self.owner.to_owned(),
            r#type: self.r#type.to_owned(),
            endpoint: self.endpoint.to_owned(),
        }
    }
}
