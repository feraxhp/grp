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
