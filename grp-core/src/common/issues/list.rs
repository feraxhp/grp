use std::fmt::Display;
use std::pin::Pin;

use futures::stream::select;
use futures::{Stream, StreamExt};

use crate::Config;
use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::Platform::Github;
use crate::animation::Animation;
use crate::common::traits::Convert;
use crate::common::utils::skip_empty;
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
        animation.change_message("getting user id...");
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
        animation.change_message("getting user id...");
        let owner = self.get_logged_user(config).await?;
        let url = self.url_list_user_assigned_issues(&config.endpoint, &owner);
        
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
    
    pub async fn list_all_involved_user_issues<'a ,A>(&'a self,
        config: &'a Config,
        animation: &'a Box<A>
    ) -> Result<Pin<Box<dyn Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>> + Send + 'a>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        match self {
            Github => {
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
                        .boxed()
                )
            },
            Platform::Gitlab => {
                let issues = gitlab::todo::list::list_todos::<gitlab::parser::Issue, A, &str>(self, Some("Issues"), config, animation).await?;
                let todos = issues.map(|response| -> Result<PaginatorResult<Vec<Issue>>, Error> {
                    // let result = parse::<gitlab::parser::Issue>(response)?;
                    let result = response?;
                    let issues = &result.result;
                    
                    let issues: Vec<Issue> = issues.into_iter().map(|b| {
                        b.target.convert()
                    }).collect();
                    
                    Ok(result.parse_result(issues))
                });
                
                let assigned_issues = self.list_user_assigned_issues(config, animation).await?;
                
                let union = select(todos, assigned_issues)
                    .filter(skip_empty);
                
                Ok(union.boxed())
            },
            Platform::Gitea |
            Platform::Codeberg |
            Platform::Forgejo => return Err(Unsupported::action("list_all_involved_user_issues", self)),
        }
    }

    pub async fn list_all_user_issues<'a, A>(&'a self,
        config: &'a Config,
        animation: &'a Box<A>
    ) -> Result<Pin<Box<dyn Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>> + Send + 'a>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        match self {
            Platform::Github |
            Platform::Gitea |
            Platform::Codeberg |
            Platform::Forgejo => {
                animation.change_message("getting user id...");
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
                        .boxed()
                )
            },
            Platform::Gitlab => {
                let scopes = [ 
                    gitlab::issues::list::Scope::AssignedToMe,
                    gitlab::issues::list::Scope::CreatedByMe,
                ];
                
                let mut stream = None;
                for scope in scopes {
                    let response = gitlab::issues::list::list_issues_by_scope(self, scope, config, animation).await?;
                    let response = response.map(|result | {
                        let result = result?;
                        let issues = result.result.iter().map(|s| s.convert()).collect();
                        
                        Ok(result.parse_result(issues))
                    });
                    
                    match stream {
                        Some(other) 
                        => stream = Some(select(other, response).boxed()),
                        None => stream = Some(response.boxed()),
                    }
                }
                
                Ok(stream.unwrap().filter(skip_empty).boxed())
            },
        }
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
