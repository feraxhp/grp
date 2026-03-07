use futures::{Stream, StreamExt};

use crate::animation::Animation;
use crate::config::Config;
use crate::platform::Platform;
use crate::error::structs::Error;
use crate::common::structs::{Context, Repo, RequestType};

impl Platform {
    /// list all the repos for the given owner, if not present, returns all the repos for the default user (config).
    /// Return a stream response
    pub async fn list_repos<T: Into<String>, A: Animation + ?Sized>(&self,
        owner: Option<T>, 
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<Vec<Repo>, Error>>, Error> {
        let owner = owner.map(|o| o.into());
        let owner = owner.unwrap_or(config.user.clone());
        
        animation.change_message("getting user type");
        let user_type = self.get_user_type(&owner, &config).await?;
        
        let url = self.url_list_repos(&user_type, &config.endpoint).await;
        
        let context = Context {
            request_type: RequestType::List,
            owner: Some(user_type.get_user().name),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching repositories...");
        
        Ok(
            self.pagginate(url, &config, context)
                .map(|result| {
                    self.get_repos(result)
                })
        )
    }
    
    pub fn get_repos(&self, response: Result<String, Error>) -> Result<Vec<Repo>, Error> {
        match response {
            Ok(rs) => Repo::from_text_array(&rs, &self),
            Err(e) => Err(e),
        }
    }
}