use crate::animation::Animation;
use crate::common::structs::{Context, Repo, RequestType};
use crate::config::Config;
use crate::error::structs::Error;
use crate::platform::Platform;


impl Platform {
    /// Creates a repository for the given owner in the selected platform.
    /// 
    /// - `owner`: the name or path of the **user** or **org** that _owns_ the repo.
    /// - `repo`: a `grp_core::Repo` with the metadata for the new repo.
    /// - `config`: a `grp_core::Config`
    /// - `animation`: a struct wich implements the trait `grp_core::animation::Animation`
    /// 
    /// # Retuns
    /// a `grp_core::Repo` with the confirmation from the platform.
    /// 
    /// #Error
    /// a `grp_core::Error` containing the detail of the error. 
    pub async fn create_repo<T: Into<String>, A: Animation + ?Sized>(&self,
        owner: Option<T>, 
        repo: Repo,
        config: &Config,
        animation: &Box<A>
    ) -> Result<Repo, Error> {
        let owner = owner.map(|o| o.into());
        let owner = owner.unwrap_or(config.user.clone());
        
        animation.change_message("getting user type");
        let user_type = self.get_user_type(&owner, &config).await?;
        
        animation.change_message("Serializing repository...");
        let json = repo.as_json(&self);
        
        animation.change_message("creating repository...");
        let url = self.url_create_repo(&user_type, &config.endpoint).await;
        let result = self.post(url, true, config, &json).await?;
        
        let context = Context {
            request_type: RequestType::Create,
            owner: Some(user_type.get_user().name),
            repo: Some(repo.name), additional: None,
        };
        
        let base_message = "Failed to create repository";
        let reponse = self.unwrap(result, base_message, &config, context).await?;
        
        Repo::from_text(&reponse, &self)
    }
}