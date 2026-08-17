use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub user: Author,
    pub pullrequest: Option<Value>,
}

