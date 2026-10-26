use mynox::Error;

use crate::common::structs::RequestType;
use crate::animation::Animation;
use crate::platform::Platform;
use crate::specific::gitlab;
use crate::config::Config;


impl Platform {
    /// Deletes the repo that its given in the selected platform.
    /// 
    /// - `owner`: the name or path of the **user** or **org** of the repo to be deleted.
    /// - `config`: a `grp_core::Config`.
    /// - `permanent`: only valid for "Gitlab", it will mark for deletion and delete permanently that repo. 
    /// - `animation`: a struct wich implements the trait `grp_core::animation::Animation`
    /// 
    /// # Return
    /// `()` if the delete succed
    /// 
    /// # Error
    /// a `grp_core::Error` containing the detail of the error. 
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
            let project = gitlab::projects::get::get_project_with_path(self, &owner, &repo, config).await?;
            owner = project.id.to_string();
        }
        
        animation.change_message("generating url ...");
        let url = self.url_delete_repo(&owner, &repo, config).await;
        
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
                let project = gitlab::projects::get::get_project_with_id(self, &owner, &owner_copy, &repo, config).await?;
                gitlab::projects::delete::premanently_remove(self, &project, config).await?;
                Ok(())
            },
            (Platform::Gitlab, 202) => Ok(()),
            (_, _) => {                
                let base_message = "Error deleting repository";
                Err(self.unwrap(result, base_message, config, &RequestType::DeleteRepo { 
                    permanent: false, 
                    owner: owner_copy.to_string(),
                    repo
                }).await.unwrap_err())
            }
        }
    }
}