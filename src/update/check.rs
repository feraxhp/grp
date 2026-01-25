use crate::update::structs::Version;

use grp_core::common::structs::{Context, RequestType};
use grp_core::error::structs::Error;
use grp_core::json::JSON;
use grp_core::platform::Platform;

use crate::usettings::structs::Pconf;

pub(crate) async fn get_latest_version() -> Result<Version, Error>{
    let pconf = Pconf {
        name: "private".to_string(),
        owner: "grp".to_string(),
        r#type: "github".to_string(),
        endpoint: "api.github.com".to_string(),
        token: "".to_string()
    };
    
    let platform = Platform::matches(&pconf.r#type);
    
    let url = format!("https://{}/repos/feraxhp/grp/releases/latest", &pconf.endpoint);
    let config = pconf.to_config();

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