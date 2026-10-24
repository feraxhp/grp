use crate::Config;
use crate::platform::Platform;


impl Platform {
    pub(crate) fn url_list_orgs(&self, config: &Config) -> String {
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                format!("{}/user/orgs", self.get_base_url(config))
            },
            Platform::Gitlab => {
                format!("{}/groups?min_access_level=10", self.get_base_url(config))
            }
        }
    }
    pub(crate) fn url_delete_org<S: AsRef<str>>(&self, name: &S, config: &Config) -> String { 
        let name = name.as_ref();
        match self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => format!("{}/orgs/{}", self.get_base_url(config), name),
            Platform::Gitlab => format!("{}/groups/{}", self.get_base_url(config), name)
        }
    }
}
