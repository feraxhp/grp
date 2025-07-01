use urlencoding::encode;

use crate::girep::common::structs::{Context, RequestType};
use crate::girep::config::Config;
use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;
use crate::girep::specific::gitlab::projects::get::Project;


pub async fn premanently_remove(platform: &Platform, project: &Project, config: &Config) -> Result<(), Error> {
    let id = project.id.to_string();
    let path = encode(&project.path_with_namespace.as_str());
    
    let url = format!("{}/projects/{}?full_path={}&permanently_remove=true", platform.get_base_url(&config.endpoint), &id, path);
    let result = platform.delete(&url, config).await?;
    
    let context = Context {
        request_type: RequestType::DeletePermanent,
        owner: None, repo: Some(id),
        additional: None
    };
    
    let base_message = "Error deleting repository";
    
    if result.status().as_u16() == 202 { return Ok(()); }
    
    Err(platform.unwrap(result, base_message, &config, context).await.unwrap_err())
}