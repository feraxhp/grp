use std::pin::Pin;

use futures::{Stream, StreamExt};
use mynox::Error;

use crate::helpers::unsuported;
use crate::specific::github::parser::SearchResult;
use crate::common::utils::skip_empty;
use crate::structs::PaginatorResult;
use crate::common::traits::Convert;
use crate::animation::Animation;
use crate::structs::RequestType;
use crate::Platform::Github;
use crate::specific::github;
use crate::specific::gitlab;
use crate::structs::Issue;
use crate::Platform;
use crate::Config;
use crate::JSON;


impl Platform {
    pub async fn list_all_involved_user_issues<'a ,A>(&'a self,
        config: &'a Config,
        animation: &'a Box<A>,
        page: usize,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>> + Send + 'a>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        match self {
            Github => {
                // let owner = self.get_logged_user(config).await?;
                let url = self.url_list_all_involved_user_issues(config);
                
                animation.change_message("fetching issues...");
                Ok(
                    self.pagginate(url, config, RequestType::ListIssues, page)
                        .map(|result| {
                            self.get_seach_result(result)
                        })
                        .boxed()
                )
            },
            Platform::Gitlab => {
                let scopes = [
                    gitlab::issues::list::Scope::Todos,
                    gitlab::issues::list::Scope::AssignedToMe,
                    gitlab::issues::list::Scope::CreatedByMe,
                ];
                
                let response = gitlab::issues::list::list_issues_by_scope(self, scopes, config, animation, page).await?;
                let stream = response.map(|result| {
                    let result = result?;
                    let issues = result.result.iter().map(|s| s.convert()).collect();
                    
                    Ok(result.parse_result(issues))
                });
                
                Ok(stream.filter(skip_empty).boxed())
            },
            Platform::Gitea |
            Platform::Codeberg |
            Platform::Forgejo => Err(unsuported(self, "list_all_involved_user_issues")),
        }
    }
    
    
    
    fn get_seach_result(&self, response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Issue>>, Error> {
        let pagginator = response?;
        let issues = match self {
            Platform::Github => {
                let result: SearchResult<github::parser::Issue> = JSON::from_str(&pagginator.result)?;
                result.items.iter().filter_map(|s| {
                    match &s.pull_request {
                        Some(_) => None,
                        None => Some(s.convert()),
                    }
                }).collect()
            },
            Platform::Gitea | 
            Platform::Gitlab | 
            Platform::Codeberg | 
            Platform::Forgejo => Issue::from_text_array(&pagginator.result, &self)?,
        };
        
        Ok(pagginator.parse_result(issues))
    }
}
