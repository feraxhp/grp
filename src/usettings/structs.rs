use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pconf {
    pub name: String,
    pub owner: String,
    pub token: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usettings {
    pub default: String,
    #[serde(rename = "pconf")]
    pub pconfs: Vec<Pconf>,
}
