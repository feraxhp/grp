use std::fmt::Display;

use futures::{Stream, StreamExt};

use crate::Config;
use crate::Error;
use crate::Platform;
use crate::animation::Animation;
use crate::common::issues::structs::Issue;
use crate::specific::gitlab;
use crate::structs::Context;
use crate::structs::RequestType;


impl Platform {
    pub async fn list_issues<T, R, A>(&self,
        owner: Option<T>, 
        repo: &R,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<Vec<Issue>, Error>>, Error> 
    where 
        T: Into<String>, 
        R: Display + AsRef<str>, 
        A: Animation + ?Sized,
    {
        let owner = owner.map(|o| o.into());
        let mut owner = owner.unwrap_or(config.user.clone());

        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting project id");
            let project = gitlab::projects::get::get_project_with_path(&self, &owner, repo.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let url = self.url_list_repo_issues(&config.endpoint, &owner, &repo);
        
        let context = Context {
            request_type: RequestType::List,
            owner: Some(owner),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, &config, context)
                .map(|result| {
                    self.get_issues(result)
                })
        )
    }
    
    pub fn get_issues(&self, response: Result<String, Error>) -> Result<Vec<Issue>, Error> {
        match response {
            Ok(rs) => Issue::from_text_array(&rs, &self),
            Err(e) => Err(e),
        }
    }
}


