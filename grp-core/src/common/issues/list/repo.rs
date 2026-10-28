use std::fmt::Display;

use futures::{Stream, StreamExt};
use mynox::Error;

use crate::structs::PaginatorResult;
use crate::structs::RequestType;
use crate::animation::Animation;
use crate::specific::gitlab;
use crate::structs::Issue;
use crate::Platform;
use crate::Config;


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
            let project = gitlab::projects::get::get_project_with_path(self, &owner, repo_path.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let url = self.url_list_repo_issues(config, &owner, &repo_path);
        animation.change_message("fetching issues...");
        
        Ok(
            self.pagginate(url, config, RequestType::ListIssues, 1)
                .map(|result| {
                    self.get_issues(result)
                })
        )
    }
}
