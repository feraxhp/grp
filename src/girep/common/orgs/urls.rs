use crate::girep::platform::Platform;


impl Platform {
    pub fn url_list_orgs<S: AsRef<str>>(&self, endpoint: &S) -> String {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/user/orgs", self.get_base_url(endpoint))
            },
            Platform::Gitlab => {
                format!("{}/groups?min_access_level=10", self.get_base_url(endpoint))
            }
        }
    }
    pub fn url_delete_org<S: AsRef<str>>(&self, name: &S, endpoint: &S) -> String { 
        let name = name.as_ref();
        match self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => format!("{}/orgs/{}", self.get_base_url(endpoint), name),
            Platform::Gitlab => format!("{}/groups/{}", self.get_base_url(endpoint), name)
        }
    }
}