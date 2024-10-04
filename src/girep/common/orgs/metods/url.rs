use crate::girep::platform::Platform;

impl Platform {
    pub(crate) fn url_list_orgs(&self, endpoint: String) -> String {
        match self {
            Platform::Github |
            Platform::Gitea => { format!("{}/user/orgs", self.get_base_url(endpoint)) }
        }
    }

    pub(crate) fn url_create_org(&self, endpoint: String) -> String {
        match self {
            Platform::Github | /*=> { panic!("Not valid for GitHub") }*/
            Platform::Gitea => { format!("{}/orgs", self.get_base_url(endpoint)) }
        }
    }
}