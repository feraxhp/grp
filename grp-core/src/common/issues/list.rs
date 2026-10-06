use std::fmt::Display;

use futures::{Stream, StreamExt};

use crate::Config;
use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::Platform::Github;
use crate::animation::Animation;
use crate::common::traits::Convert;
use crate::specific::github;
use crate::specific::github::parser::SearchResult;
use crate::error::errors::unsupported::Unsupported;
use crate::specific::gitlab;
use crate::structs::Context;
use crate::structs::Issue;
use crate::structs::PaginatorResult;
use crate::structs::RequestType;


impl Platform {
    pub async fn list_issues<T, R, A>(&self,
        owner: Option<T>, 
        repo_path: &R,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
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
        
        let url = self.url_list_repo_issues(&config.endpoint, &owner, &repo_path);
        
        let context = Context {
            request_type: RequestType::ListIssues,
            owner: Some(owner),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, &config, context, 1)
                .map(|result| {
                    self.get_issues(result)
                })
        )
    }
    
    pub async fn list_user_assigned_issues<A>(&self,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        let owner = self.get_logged_user(config).await?;
        let url = self.url_list_user_assigned_issues(&config.endpoint);
        
        let context = Context {
            request_type: RequestType::ListIssues,
            owner: Some(owner.name),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, &config, context, 1)
                .map(|result| {
                    self.get_issues(result)
                })
        )
    }
    
    pub async fn list_all_involved_user_issues<A>(&self,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        if !matches!(self, Github) {
            return Err(Unsupported::action("list_all_involved_user_issues", self));
        }
        let owner = self.get_logged_user(config).await?;
        let url = self.url_list_all_involved_user_issues(&config.endpoint);
        
        let context = Context {
            request_type: RequestType::ListIssues,
            owner: Some(owner.name),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, &config, context, 1)
                .map(|result| {
                    self.get_seach_result(result)
                })
        )
    }

    pub async fn list_all_user_issues<A>(&self,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        let owner = self.get_logged_user(config).await?;
        let url = self.url_list_all_user_issues(&config.endpoint);
        
        let context = Context {
            request_type: RequestType::ListIssues,
            owner: Some(owner.name),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, &config, context, 1)
                .map(|result| {
                    self.get_issues(result)
                })
        )
    }
    
    pub fn get_issues(&self, response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Issue>>, Error> {
        let pagginator = response?;
        let issues = Issue::from_text_array(&pagginator.result, &self)?;
        Ok(pagginator.parse_result(issues))
    }
    
    fn get_seach_result(&self, response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Issue>>, Error> {
        let pagginator = response?;
        let issues = match self {
            Platform::Github => {
                let result: SearchResult<github::parser::Issue> = JSON::from_str(&pagginator.result)?;
                result.items.iter().map(|s| s.convert()).collect()
            },
            Platform::Gitea | 
            Platform::Gitlab | 
            Platform::Codeberg | 
            Platform::Forgejo => Issue::from_text_array(&pagginator.result, &self)?,
        };
        
        Ok(pagginator.parse_result(issues))
    }
}


