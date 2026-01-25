use crate::platform::Platform;
use crate::error::structs::Error;
use crate::config::Config;
use crate::common::structs::{Context, RequestType};
use crate::animation::Animation;
use crate::specific::gitlab;


impl Platform {
    pub async fn delete_org<T: Into<String>, A: Animation + ?Sized>(&self,
        name: T,
        config: &Config,
        permanent: bool,
        animation: &Box<A>
    ) -> Result<(), Error> {
        let mut name = name.into();
        let name_copy = name.clone();
        
        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting group id");
            let user = gitlab::groups::get::get_group_with_path(self, &name, config).await?;
            name = user.id;
        }
        
        animation.change_message("generating url ...");
        let url = self.url_delete_org(&name, &config.endpoint);
        
        animation.change_message("Deleting organization ...");
        let result = self.delete(&url, config).await?;
        
        match (self, result.status().as_u16()) {
            (Platform::Gitlab, 202 | 400) if permanent => {
                animation.change_message("Permamently deleting gitlab group ...");
                let user = gitlab::groups::get::get_group_by_id(self, &name, config).await?;
                let _ = gitlab::groups::delete::premanently_remove(&self, &user, config).await?;
                Ok(())
            },
            (
                Platform::Codeberg |
                Platform::Forgejo
                , 204
            ) | (_, 202) => Ok(()),
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