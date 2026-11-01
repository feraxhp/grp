use std::pin::Pin;

use futures::{Stream, StreamExt};

use crate::Config;
use mynox::Error;
use crate::Platform;
use crate::animation::Animation;
use crate::common::traits::Convert;
use crate::common::utils::skip_empty;
use crate::specific::{forgejo, github, gitlab};
use crate::structs::Issue;
use crate::structs::PaginatorResult;


impl Platform {
    /// Allows you to retrieve all the issues assosiated with an acount. 
    /// 1. for github uses the filter All
    /// 2. for gitlab unifies the request from _AssignedToMe, CreatedByMe_ (this may return duplicates)
    /// 3. for forgejo (and compatibles) unifies the request from _AssignedToMe, CreatedByMe, Mentioned_ (this may return duplicates)
    pub async fn list_all_user_issues<'a, A>(&'a self,
        config: &'a Config,
        animation: &'a Box<A>
    ) -> Result<Pin<Box<dyn Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>> + Send + 'a>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        match self {
            Platform::Github => {
                let issues = github::issues::list::list_issues_by_scope(
                    self, 
                    github::issues::list::Filter::All,
                    config, 
                    animation
                ).await?;

                Ok(
                issues
                    .map(|result| { 
                        let pagginator = result?;
                        let issues = pagginator.result.iter().filter_map(|s| {
                            match &s.pull_request {
                                Some(_) => None,
                                None => Some(s.convert()),
                            }
                        }).collect();
                        
                        Ok(pagginator.parse_result(issues))
                    })
                    .boxed()
                )
            },
            Platform::Gitlab => {
                let scopes = [ 
                    gitlab::issues::list::Scope::AssignedToMe,
                    gitlab::issues::list::Scope::CreatedByMe,
                ];
                
                let response = gitlab::issues::list::list_issues_by_scope(self, scopes, config, animation).await?;
                let stream = response.map(|result| {
                    let result = result?;
                    let issues = result.result.iter().map(|s| s.convert()).collect();
                    
                    Ok(result.parse_result(issues))
                });
                
                Ok(stream.filter(skip_empty).boxed())
            },
            Platform::Forgejo |
            Platform::Gitea |
            Platform::Codeberg => {
                let scopes = [ 
                    forgejo::issues::list::Scope::AssignedToMe,
                    forgejo::issues::list::Scope::CreatedByMe,
                    forgejo::issues::list::Scope::Mentioned,
                ];
                
                let response = forgejo::issues::list::list_issues_by_scope(self, scopes, config, animation).await?;
                let stream = response.map(|result| {
                    let result = result?;
                    let issues = result.result.iter().filter_map(|s| {
                        match &s.pull_request {
                            Some(_) => None,
                            None => Some(s.convert()),
                        }
                    }).collect();
                    
                    Ok(result.parse_result(issues))
                });
                
                Ok(stream.filter(skip_empty).boxed())
            },
        }
    }
}
