use std::fmt::Display;

use futures::{Stream, StreamExt};

use crate::Config;
use crate::animation::Animation;
use crate::specific::gitlab;
use crate::structs::Comment;
use crate::structs::Context;
use crate::Platform;
use crate::Error;
use crate::structs::Issue;
use crate::structs::RequestType;


impl Platform {
    pub async fn list_issue_comments<A, R, T>(&self,
        owner: Option<T>, 
        repo_path: &R,
        issue: &u64,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<Vec<Comment>, Error>>, Error> 
    where 
        T: Into<String>, 
        R: Display + AsRef<str>, 
        A: Animation + ?Sized,
    {
        let owner = owner.map(|o| o.into());
        let mut owner = owner.unwrap_or(config.user.clone());
        let repo = format!("{owner}/{repo_path}"); 
        
        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting project id");
            let project = gitlab::projects::get::get_project_with_path(&self, &owner, repo_path.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let url = self.url_repo_issues_comments(&config.endpoint, &owner, &repo_path, issue);
        
        let context = Context {
            request_type: RequestType::ListIssuesComments,
            owner: Some(owner.clone()),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issue comments...");
        
        Ok(
            self.pagginate(url, &config, context)
                .map(move |result| {
                    let repo = repo.clone();
                    self.get_comments(result, repo)
                })
        )
    }

    pub fn get_comments(&self, response: Result<String, Error>, repo: String) -> Result<Vec<Comment>, Error> {
        match response {
            Ok(rs) => Comment::from_text_array(&rs, &self, repo),
            Err(e) => Err(e),
        }
    }
    
    pub async fn issue<A, R, T>(&self,
        owner: Option<T>, 
        repo_path: &R,
        issue: &u64,
        config: &Config,
        animation: &Box<A>
    ) -> Result<Issue, Error>
    where 
        T: Into<String>, 
        R: Display + AsRef<str>, 
        A: Animation + ?Sized,
    {
        let owner = owner.map(|o| o.into());
        let mut owner = owner.unwrap_or(config.user.clone());

        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting project id");
            let project = gitlab::projects::get::get_project_with_path(&self, &owner, repo_path.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let url = self.url_detailed_issue(&config.endpoint, &owner, &repo_path, issue);
        let result = self.get(url, true, config).await?;
        let context = Context {
            request_type: RequestType::ListIssuesComments,
            owner: Some(owner.clone()),
            repo: None,
            additional: None,
        };
        
        let base_message = "Failed to fetch issue";
        let result = self.unwrap(result, base_message, &config, context).await?;
        
        Ok(
            Issue::from_text_array(&format!("[{result}]"), self)?[0].clone()
        )
    }
}

