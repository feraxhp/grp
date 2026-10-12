use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pconf {
    pub name: String,
    pub owner: String,
    pub token: Zeroizing<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub endpoint: String,
    #[serde(default)]
    pub encripted: bool,
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
