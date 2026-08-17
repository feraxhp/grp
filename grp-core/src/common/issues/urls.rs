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
    
}