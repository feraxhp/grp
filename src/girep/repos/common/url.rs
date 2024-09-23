use crate::girep::config::Config;
use crate::girep::repos::common::supported::Platform;
use crate::girep::repos::common::utype::UserType;

impl Platform {
    pub(crate) fn url_list_repos(&self, owner: String, user_type: UserType, endpoint: String) -> String {
        match self {
            Platform::Github |
            Platform::Gitea => {
                match user_type {
                    UserType::Free => format!("{}/users/{}/repos", self.get_base_url(endpoint), owner),
                    UserType::Organization => format!("{}/orgs/{}/repos", self.get_base_url(endpoint), owner),
                    UserType::Logged => format!("{}/user/repos", self.get_base_url(endpoint))

                }
            }
            _ => todo!("Not implemented")
        }
    }
    pub(crate) fn url_delete_repo(&self, owner: String, repo: String, endpoint: String) -> String {
        match self {
            Platform::Github |
            Platform::Gitea => format!("{}/repos/{}/{}", self.get_base_url(endpoint), owner, repo),
            _ => todo!("Not implemented")
        }
    }
}