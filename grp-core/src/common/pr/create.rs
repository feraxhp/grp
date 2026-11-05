use mynox::Error;
use serde::Serialize;

use crate::structs::PullRequestOptions;
use crate::structs::RequestType;
use crate::structs::PullRequest;
use crate::animation::Animation;
use crate::specific::gitlab;
use crate::Platform;
use crate::Config;


impl Platform {
    pub async fn create_pr<A>(&self, 
        options: &PullRequestOptions, 
        config: &Config,
        animation: &Box<A>,
    ) -> Result<PullRequest, Error> 
    where 
        A: Animation + ?Sized
    {
        let url = self.url_create_pr(config, options);
        let mut target_id = 0;
        
        if matches!(self, Platform::Gitlab) {
            animation.change_message("getting target project id");
            target_id = gitlab::projects::get::get_project_with_path(self, &options.target.owner, &options.target.repo, config).await?.id;
        };
        
        animation.change_message("Creating pull request");
        let result = self.post(url, true, config, &self.payload(options, target_id)).await?;
        let text = self.unwrap(result, "Faild to create the pull request", config, &RequestType::CreatePr(options.clone())).await?;
        
        Ok(PullRequest::from_text(&text, self)?)
    }
    
    fn payload<'a>(&self, options: &'a PullRequestOptions, target_id: u64) -> PrPayload<'a> {
        match self {
            Platform::Gitlab => PrPayload::GitLab(GitLabPayload {
                title: &options.title,
                description: &options.body,
                source_branch: &options.origin.branch,
                target_branch: &options.target.branch,
                target_project_id: target_id,
                allow_collaboration: options.maintainer_can_modify,
            }),
            Platform::Gitea
            | Platform::Codeberg
            | Platform::Forgejo => PrPayload::Forgejo(ForgejoPayload {
                title: &options.title,
                body: &options.body,
                head: format!("{}:{}", options.origin.owner, options.origin.branch),
                base: &options.target.branch,
            }),
            Platform::Github => PrPayload::Github(GithubPayload {
                title: &options.title,
                body: &options.body,
                head: format!("{}:{}", options.origin.owner, options.origin.branch),
                base: &options.target.branch,
                draft: options.draft,
                maintainer_can_modify: options.maintainer_can_modify,
            }),
        }
    }

}

#[derive(Serialize)]
#[serde(untagged)]
enum PrPayload<'a> {
    GitLab(GitLabPayload<'a>),
    Github(GithubPayload<'a>),
    Forgejo(ForgejoPayload<'a>),
}

#[derive(Serialize)]
struct GitLabPayload<'a> {
    title: &'a str,
    description: &'a str,
    source_branch: &'a str,
    target_branch: &'a str,
    target_project_id: u64,
    allow_collaboration: bool,
}

#[derive(Serialize)]
struct GithubPayload<'a> {
    title: &'a str,
    body: &'a str,
    head: String,
    base: &'a str,
    draft: bool,
    maintainer_can_modify: bool,
}

#[derive(Serialize)]
struct ForgejoPayload<'a> {
    title: &'a str,
    body: &'a str,
    head: String,
    base: &'a str,
}
