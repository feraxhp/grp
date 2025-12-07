use crate::girep::animation::Animation;
use crate::girep::common::structs::{Context, Repo, RequestType};
use crate::girep::config::Config;
use crate::girep::error::structs::Error;
use crate::girep::platform::Platform;


impl Platform {
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