use mynox::Error;

use grp_core::structs::RequestType;
use grp_core::Platform;
use grp_core::Config;
use grp_core::JSON;

use crate::update::structs::Version;


pub(crate) async fn get_latest_version() -> Result<Version, Error> {
    let platform = Platform::Github;
    let config = Config::new("internal", "grp", "", "api.github.com");
    
    let url = format!("https://{}/repos/feraxhp/grp/releases/latest", config.endpoint);
    
    let result = platform.get(url, false, &config).await?;
    
    let context = RequestType::RepositoryDetails { owner: "feraxhp".to_string(), repo: "grp".to_string() }; 
    
    let base_message = "Error getting the latest realease";
    let response = platform.unwrap(result, base_message, &config, &context).await?;
    
    let version: Version = JSON::from_str(&response)?;
    
    Ok(version)
}
