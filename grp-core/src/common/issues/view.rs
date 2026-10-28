use std::fmt::Display;

use futures::{Stream, StreamExt};
use mynox::Error;

use crate::structs::PaginatorResult;
use crate::animation::Animation;
use crate::structs::RequestType;
use crate::specific::gitlab;
use crate::structs::Comment;
use crate::structs::Issue;
use crate::Platform;
use crate::Config;


impl Platform {
    pub async fn list_issue_comments<A, R, T>(&self,
        owner: Option<T>, 
        repo_path: &R,
        issue: &u64,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Comment>>, Error>>, Error> 
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
            let project = gitlab::projects::get::get_project_with_path(self, &owner, repo_path.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let url = self.url_repo_issues_comments(config, &owner, &repo_path, issue);
        
        animation.change_message("fetching issue comments...");
        
        Ok(
            self.pagginate(url, config, RequestType::ListIssuesComments { owner, repo: repo.clone(), number: *issue }, 1)
                .map(move |result| {
                    let repo = repo.clone();
                    self.get_comments(result, repo, config, issue)
                })
        )
    }

    pub fn get_comments(&self, response: Result<PaginatorResult<String>, Error>, repo: String, config: &Config, issue: &u64) -> Result<PaginatorResult<Vec<Comment>>, Error> {
        let pagginator = response?;
        let comments = Comment::from_text_array(&pagginator.result, self, repo, config, *issue)?;
        Ok(pagginator.parse_result(comments))
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
            let project = gitlab::projects::get::get_project_with_path(self, &owner, repo_path.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let url = self.url_detailed_issue(config, &owner, &repo_path, issue);
        let result = self.get(url, true, config).await?;
        
        let base_message = "Failed to fetch issue";
        let result = self.unwrap(result, base_message, config, &RequestType::IssueDetails { owner, repo: repo_path.to_string(), number: *issue }).await?;
        
        Ok(Issue::from_text(&result, self)?)
    }
}
