use std::fmt::Display;

use crate::Platform;




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
    
    pub(crate) fn url_list_user_assigned_issues<S>(&self, endpoint: &S) -> String 
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
                format!("{}/issues?state=opened&scope=assigned_to_me", self.get_base_url(endpoint))
            },
            Platform::Gitlab => {
                format!("{}/repos/issues/search?state=open&assigned=true&type=issues", self.get_base_url(endpoint))
            }
        }
    }
    
    pub(crate) fn url_list_all_user_issues<S>(&self, endpoint: &S) -> String 
    where 
        S: AsRef<str>,
    {
        match &self {
            Platform::Github => {
                format!("{}/user/issues?state=open&filter=all", self.get_base_url(endpoint))
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/issues?state=opened&scope=all", self.get_base_url(endpoint))
            },
            Platform::Gitlab => {
                format!("{}/repos/issues/search?state=open&type=issues", self.get_base_url(endpoint))
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