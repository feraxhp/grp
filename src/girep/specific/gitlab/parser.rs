use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repository {
    pub path: String,
    pub path_with_namespace: String,
    pub web_url: String,
    pub http_url_to_repo: String,
    pub visibility: String,
    pub description: Option<String>,
}
