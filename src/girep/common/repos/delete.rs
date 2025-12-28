use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;
use crate::girep::config::Config;
use crate::girep::common::structs::{Context, RequestType};
use crate::girep::animation::Animation;
use crate::girep::specific::gitlab;


impl Platform {
    pub async fn delete_repo<T: Into<String>, A: Animation + ?Sized>(&self,
        owner: T, repo: T,
        config: &Config,
        permanent: bool,
        animation: &Box<A>
    ) -> Result<(), Error> {
        let mut owner = owner.into(); let repo = repo.into();
        let owner_copy = owner.clone();
        
        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting project id");
            let project = gitlab::projects::get::get_project_with_path(&self, &owner, &repo, config).await?;
            owner = project.id.to_string();
        }
        
        animation.change_message("generating url ...");
        let url = self.url_delete_repo(&owner, &repo, &config.endpoint).await;
        
        animation.change_message("Deleting repository ...");
        let result = self.delete(&url, config).await?;
        
        match (self, result.status().as_u16()) {
            (
                Platform::Gitea |
                Platform::Codeberg |
                Platform::Forgejo | 
                Platform::Github, 204
            ) => Ok(()),
            (Platform::Gitlab, 202 | 400) if permanent => {
                animation.change_message("Permamently deleting gitlab project ...");
                let project = gitlab::projects::get::get_project_with_id(&self, &owner, config).await?;
                let _ = gitlab::projects::delete::premanently_remove(&self, &project, config).await?;
                Ok(())
            },
            (Platform::Gitlab, 202) => Ok(()),
            (_, _) => {
                let context = Context {
                    request_type: RequestType::Delete,
                    owner: Some(owner_copy),
                    repo: Some(repo),
                    additional: None
                };
                
                let base_message = "Error deleting repository";
                Err(self.unwrap(result, base_message, &config, context).await.unwrap_err())
            }
        }
    }
}