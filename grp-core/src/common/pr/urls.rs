use crate::structs::PullRequestOptions;
use crate::Platform;
use crate::Config;


impl Platform {
    pub(crate) fn url_create_pr(&self, config: &Config, options: &PullRequestOptions) -> String {
        match self {
            Platform::Gitlab => {
                let target_path = format!("{}/{}", options.target.owner, options.target.repo);
                let encoded_target: String = url::form_urlencoded::byte_serialize(target_path.as_bytes()).collect();
                format!("{}/api/v4/projects/{}", self.get_base_url(config), encoded_target)
            },
            Platform::Gitea |
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo => format!("{}/repos/{}/{}/pulls", self.get_base_url(config), options.origin.owner, options.origin.repo),
        }
    }
}

