pub mod comment;
pub mod issue;
pub mod group;
pub mod user;


use std::fmt::Debug;

use serde::Deserialize;
use serde::de::DeserializeOwned;
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
pub struct User {
    pub id: u64,
    pub username: String,
    pub web_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub web_url: String,
    pub full_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub iid: u64,
    pub author: User,
    pub title: String,
    pub description: Option<String>,
    // pub issue_type: String,
    pub web_url: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub author: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
#[allow(unused)]
pub struct Todos<T> 
where 
    T: Debug,
{
    pub target_type: String,
    pub target: T
}
