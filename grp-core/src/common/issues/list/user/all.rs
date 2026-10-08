use std::pin::Pin;

use futures::stream::select;
use futures::{Stream, StreamExt};

use crate::Config;
use crate::Error;
use crate::Platform;
use crate::animation::Animation;
use crate::common::traits::Convert;
use crate::common::utils::skip_empty;
use crate::specific::gitlab;
use crate::structs::Context;
use crate::structs::Issue;
use crate::structs::PaginatorResult;
use crate::structs::RequestType;


impl Platform {
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
}