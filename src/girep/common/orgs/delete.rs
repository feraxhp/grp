use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;
use crate::girep::config::Config;
use crate::girep::common::structs::{Context, RequestType};
use crate::girep::animation::Animation;
use crate::girep::specific::gitlab;


impl Platform {
    pub async fn delete_org<T: Into<String>, A: Animation + ?Sized>(&self,
        name: T,
        config: &Config,
        permanent: bool,
        animation: Option<&Box<A>>
    ) -> Result<(), Error> {
        let mut name = name.into();
        let name_copy = name.clone();
        
        if matches!(self, Platform::Gitlab) {
            if let Some(an) = animation { an.change_message("getting group id"); }
            let user = gitlab::groups::get::get_group_with_path(self, &name, config).await?;
            name = user.id;
        }
        
        if let Some(an) = animation { an.change_message("generating url ..."); }
        let url = self.url_delete_org(&name, &config.endpoint);
        
        if let Some(an) = animation { an.change_message("Deleting repository ..."); }
        let result = self.delete(&url, config).await?;
        
        match (self, result.status().as_u16()) {
            (Platform::Gitlab, 202 | 400) if permanent => {
                if let Some(an) = animation { an.change_message("Permamently deleting gitlab group ..."); }
                let user = gitlab::groups::get::get_group_by_id(self, &name, config).await?;
                let _ = gitlab::groups::delete::premanently_remove(&self, &user, config).await?;
                Ok(())
            },
            (Platform::Codeberg, 204) => Ok(()),
            (_, 202) => Ok(()),
            (_, _) => {
                let context = Context {
                    request_type: RequestType::DeleteOrg,
                    owner: Some(name_copy),
                    repo: None,
                    additional: None
                };
                
                let base_message = "Error deleting organization";
                Err(self.unwrap(result, base_message, &config, context).await.unwrap_err())
            }
        }
    }
}