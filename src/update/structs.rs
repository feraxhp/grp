use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) struct Version {
    pub name: String,
    pub assets: Vec<Asset>
}

#[derive(Deserialize, Clone)]
pub(crate) struct Asset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub archive: String
}
