use futures::{Stream, StreamExt};

use crate::Config;
use crate::animation::Animation;
use crate::specific::gitlab;
use crate::structs::Comment;
use crate::structs::Context;
use crate::Platform;
use crate::Error;
use crate::structs::RequestType;


impl Platform {
    pub async fn list_issue_comments<A, T>(&self,
        owner: Option<T>, 
        repo: &str,
        issue: &u64,
        config: &Config,
        animation: &Box<A>
    ) -> Result<impl Stream<Item = Result<Vec<Comment>, Error>>, Error> 
    where 
        T: Into<String>, 
        A: Animation + ?Sized,
    {
        let owner = owner.map(|o| o.into());
        let mut owner = owner.unwrap_or(config.user.clone());

        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting project id");
            let project = gitlab::projects::get::get_project_with_path(&self, &owner, repo.as_ref(), config).await?;
            owner = project.id.to_string();
        }
        
        let issue = issue;
        // if matches!(self, Platform::Gitlab) {
        //     animation.change_message("getting project id");
        //     let issue = gitlab::issues::get::get_issue_iid(&self, &owner, repo.as_ref(), issue, config).await?;
        //     owner = project.id.to_string();
        // }
        
        let url = self.url_repo_issues_comments(&config.endpoint, &owner, &repo, issue);
        
        let context = Context {
            request_type: RequestType::ListIssuesComments,
            owner: Some(owner),
            repo: None,
            additional: None,
        };
        
        animation.change_message("fetching issue comments...");
        
        Ok(
            self.pagginate(url, &config, context)
                .map(|result| {
                    self.get_comments(result)
                })
        )
    }

    pub fn get_comments(&self, response: Result<String, Error>) -> Result<Vec<Comment>, Error> {
        match response {
            Ok(rs) => Comment::from_text_array(&rs, &self),
            Err(e) => Err(e),
        }
    }
}

