use serde::Deserialize;
use urlencoding::encode;

use crate::platform::Platform;
use crate::json::JSON;
use crate::error::structs::Error;
use crate::config::Config;
use crate::common::structs::{Context, RequestType};

#[derive(Deserialize, Clone)]
pub struct Project {
    pub id: u64,
    pub path_with_namespace: String,
}

pub async fn get_project_with_path(platform: &Platform, owner: &str, repo: &str, config: &Config) -> Result<Project, Error> {
    let path = format!("{}/{}", owner, repo);
    let path  = encode(&path);
    let url = format!("{}/projects/{}", platform.get_base_url(&config.endpoint), path);
    
    get_project(platform, &url, config).await
}

pub async fn get_project_with_id(platform: &Platform, id: &str, config: &Config) -> Result<Project, Error> {
    let url = format!("{}/projects/{}", platform.get_base_url(&config.endpoint), id);
    
    get_project(platform, &url, config).await
}

pub async fn get_project(platform: &Platform, url: &str, config: &Config) -> Result<Project, Error> {
    let result = platform.get(url, true, config).await?;
    
    let base_message = "Error geting the project id";
    
    let context = Context {
        request_type: RequestType::RepositoryDetails,
        owner: None, repo: None, additional: None
    };
    
    let result = platform.unwrap(result, base_message, config, context).await?;
    
    let pj: Project = JSON::from_str(&result)?;
    
    Ok(pj)
}