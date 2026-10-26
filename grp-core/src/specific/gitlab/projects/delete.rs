use urlencoding::encode;
use mynox::Error;

use crate::specific::gitlab::projects::get::Project;
use crate::common::structs::RequestType;
use crate::platform::Platform;
use crate::config::Config;


pub async fn premanently_remove(platform: &Platform, project: &Project, config: &Config) -> Result<(), Error> {
    let id = project.id.to_string();
    let path = encode(project.path_with_namespace.as_str());
    
    let url = format!("{}/projects/{}?full_path={}&permanently_remove=true", platform.get_base_url(config), id, path);
    let result = platform.delete(&url, config).await?;
    
    let base_message = "Error deleting repository";
    
    if result.status().as_u16() == 202 { return Ok(()); }
    
    Err(platform.unwrap(result, base_message, config, &RequestType::DeleteRepo { 
        permanent: true, 
        owner: project.namespace.full_path.to_string(), 
        repo: project.name.to_string(), 
    }).await.unwrap_err())
}
