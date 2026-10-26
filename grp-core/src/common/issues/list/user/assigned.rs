use futures::{Stream, StreamExt};
use mynox::Error;

use crate::structs::PaginatorResult;
use crate::structs::RequestType;
use crate::animation::Animation;
use crate::structs::Issue;
use crate::Platform;
use crate::Config;


impl Platform {
    pub async fn list_user_assigned_issues<A>(&self,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
    where 
        A: Animation + ?Sized,
    {
        animation.change_message("getting user id...");
        let owner = self.get_logged_user(config).await?;
        let url = self.url_list_user_assigned_issues(config, &owner);
        
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, config, RequestType::ListIssues, 1)
                .map(|result| {
                    self.get_issues(result)
                })
        )
    }
}