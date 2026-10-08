use std::pin::Pin;

use futures::stream::select;
use futures::{Stream, StreamExt};

use crate::Config;
use crate::Error;
use crate::Platform;
use crate::animation::Animation;
use crate::common::traits::Convert;
use crate::common::utils::skip_empty;
use crate::specific::{gitea, github, gitlab};
use crate::structs::Issue;
use crate::structs::PaginatorResult;


impl Platform {
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
                        let issues = pagginator.result.iter().map(|s| s.convert()).collect();
                        
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
            Platform::Forgejo |
            Platform::Gitea |
            Platform::Codeberg => {
                let scopes = [ 
                    gitea::issues::list::Scope::AssignedToMe,
                    gitea::issues::list::Scope::CreatedByMe,
                    gitea::issues::list::Scope::Mentioned,
                ];
                
                let mut stream = None;
                for scope in scopes {
                    let response = gitea::issues::list::list_issues_by_scope(self, scope, config, animation).await?;
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
}