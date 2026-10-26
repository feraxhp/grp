use urlencoding::encode;
use serde::Deserialize;
use mynox::Error;

use crate::common::structs::RequestType;
use crate::platform::Platform;
use crate::config::Config;
use crate::json::JSON;


#[derive(Deserialize, Clone)]
pub struct Namespace {
    // pub id: u64,
    pub full_path: String,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub id: u64,
    pub name: String, 
    pub namespace: Namespace,
    pub path_with_namespace: String,
}

pub async fn get_project_with_path(platform: &Platform, owner: &str, repo: &str, config: &Config) -> Result<Project, Error> {
    let path = format!("{}/{}", owner, repo);
    let path  = encode(&path);
    let url = format!("{}/projects/{}", platform.get_base_url(config), path);
    
    get_project(platform, &url, owner, repo, config).await
}

pub async fn get_project_with_id(platform: &Platform, id: &str, owner: &str, repo: &str, config: &Config) -> Result<Project, Error> {
    let url = format!("{}/projects/{}", platform.get_base_url(config), id);
    
    get_project(platform, &url, owner, repo, config).await
}

pub async fn get_project(platform: &Platform, url: &str, owner: &str, repo: &str, config: &Config) -> Result<Project, Error> {
    let result = platform.get(url, true, config).await?;
    
    let base_message = "Error geting the project id";
    
    let result = platform.unwrap(result, base_message, config, &RequestType::RepositoryDetails { owner: owner.to_string(), repo: repo.to_string() }).await?;
    
    let pj: Project = JSON::from_str(&result)?;
    
    Ok(pj)
}