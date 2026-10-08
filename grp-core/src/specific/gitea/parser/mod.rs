pub mod comment;
pub mod issue;
pub mod user;

use serde::Deserialize;
use serde_json::Value;
use chrono::Utc;
use chrono::DateTime;

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
pub struct User {
    pub login: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub pull_request: Option<Value>,
    pub html_url: String,
    pub state: String,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub user: User,
    pub body: String,
    pub html_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
