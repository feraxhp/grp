use crate::update::structs::Version;

use grp_core::structs::{Context, RequestType};
use grp_core::{Config, Error, JSON, Platform};

pub(crate) async fn get_latest_version() -> Result<Version, Error> {
    let platform = Platform::Github;
    let config = Config::new("internal", "grp", "", "api.github.com");
    
    let url = format!("https://{}/repos/feraxhp/grp/releases/latest", &config.endpoint);

    let result = platform.get(url, false, &config).await?;
    
    let context = Context {
        request_type: RequestType::RepositoryDetails,
        owner: Some(config.user.to_string()),
        repo: None, additional: None,
    };
    let base_message = "Error getting the latest realease";
    let response = platform.unwrap(result, base_message, &config, context).await?;

    let version: Version = JSON::from_str(&response)?;

    Ok(version)
}