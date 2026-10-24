use std::fmt::Display;

use crate::{Config, Platform, structs::User};




impl Platform {
    pub(crate) fn url_list_repo_issues<O, R>(&self, config: &Config, owner: &O, repo: &R) -> String 
    where 
        O: Display,
        R: Display,
    {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/{}/{}/issues", self.get_base_url(config), owner, repo)
            },
            Platform::Gitlab => {
                format!("{}/projects/{}/issues", self.get_base_url(config), owner)
            }
        }
    }
    
    pub(crate) fn url_list_user_assigned_issues(&self, config: &Config, assignee: &User) -> String {
        match &self {
            Platform::Github => {
                format!("{}/user/issues?state=open&filter=assigned", self.get_base_url(config))
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/issues/search?state=open&assigned=true&type=issues", self.get_base_url(config))
            },
            Platform::Gitlab => {
                format!("{}/issues?state=opened&assignee_id={}", self.get_base_url(config), assignee.id)
            }
        }
    }
    
    
    pub(crate) fn url_list_all_involved_user_issues(&self, config: &Config) -> String {
        match &self {
            Platform::Github => {
                format!("{}/search/issues?q=is:issue+state:open+involves:@me", self.get_base_url(config))
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
    
    pub(crate) fn url_detailed_issue<O, R>(&self, config: &Config, owner: &O, repo: &R, issue: &u64) -> String 
    where 
        O: Display,
        R: Display,
    {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/{}/{}/issues/{}", self.get_base_url(config), owner, repo, issue)
            },
            Platform::Gitlab => {
                format!("{}/projects/{}/issues/{}", self.get_base_url(config), owner, issue)
            }
        }
    }
    
    pub(crate) fn url_repo_issues_comments<O, R>(&self, config: &Config, owner: &O, repo: &R, issue: &u64) -> String 
    where 
        O: Display,
        R: Display,
    {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/repos/{}/{}/issues/{}/comments", self.get_base_url(config), owner, repo, issue)
            },
            Platform::Gitlab => {
                format!("{}/projects/{}/issues/{}/notes?sort=asc", self.get_base_url(config), owner, issue)
            }
        }
    }
    
}