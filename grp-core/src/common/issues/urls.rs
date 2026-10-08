use std::fmt::Display;

use crate::{Platform, structs::User};




impl Platform {
    pub(crate) fn url_list_repo_issues<S, O, R>(&self, endpoint: &S, owner: &O, repo: &R) -> String 
    where 
        S: AsRef<str>,
        O: Display,
        R: Display,
    {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/{}/{}/issues", self.get_base_url(endpoint), owner, repo)
            },
            Platform::Gitlab => {
                format!("{}/projects/{}/issues", self.get_base_url(endpoint), owner)
            }
        }
    }
    
    pub(crate) fn url_list_user_assigned_issues<S>(&self, endpoint: &S, assignee: &User) -> String 
    where 
        S: AsRef<str>,
    {
        match &self {
            Platform::Github => {
                format!("{}/user/issues?state=open&filter=assigned", self.get_base_url(endpoint))
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/issues/search?state=open&assigned=true&type=issues", self.get_base_url(endpoint))
            },
            Platform::Gitlab => {
                format!("{}/issues?state=opened&assignee_id={}", self.get_base_url(endpoint), assignee.id)
            }
        }
    }
    
    
    pub(crate) fn url_list_all_involved_user_issues<S>(&self, endpoint: &S) -> String 
    where 
        S: AsRef<str>,
    {
        match &self {
            Platform::Github => {
                format!("{}/search/issues?q=is:issue+state:open+involves:@me", self.get_base_url(endpoint))
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                String::new()
            },
            Platform::Gitlab => {
                String::new()
            }
        }
    }
    
    pub(crate) fn url_detailed_issue<S, O, R>(&self, endpoint: &S, owner: &O, repo: &R, issue: &u64) -> String 
    where 
        S: AsRef<str>,
        O: Display,
        R: Display,
    {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/{}/{}/issues/{}", self.get_base_url(endpoint), owner, repo, issue)
            },
            Platform::Gitlab => {
                format!("{}/projects/{}/issues/{}", self.get_base_url(endpoint), owner, issue)
            }
        }
    }
    
    pub(crate) fn url_repo_issues_comments<S, O, R>(&self, endpoint: &S, owner: &O, repo: &R, issue: &u64) -> String 
    where 
        S: AsRef<str>,
        O: Display,
        R: Display,
    {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/{}/{}/issues/{}/comments", self.get_base_url(endpoint), owner, repo, issue)
            },
            Platform::Gitlab => {
                format!("{}/projects/{}/issues/{}/notes?sort=asc", self.get_base_url(endpoint), owner, issue)
            }
        }
    }
    
}