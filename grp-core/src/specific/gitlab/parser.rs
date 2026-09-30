use serde::Deserialize;
use chrono::Utc;
use chrono::DateTime;

#[derive(Deserialize)]
pub struct Repository {
    pub path: String,
    pub path_with_namespace: String,
    pub web_url: String,
    pub http_url_to_repo: String,
    pub visibility: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub iid: u64,
    pub author: Author,
    pub title: String,
    pub issue_type: String,
    pub web_url: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub author: Author,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
