use crate::structs::PullRequestOptions;
use crate::Platform;
use crate::Config;


impl Platform {
    pub(crate) fn url_create_pr(&self, config: &Config, options: &PullRequestOptions) -> String {
        match self {
            Platform::Gitlab => {
                let origin_path = format!("{}/{}", options.origin.owner, options.origin.repo);
                let encoded_target: String = url::form_urlencoded::byte_serialize(origin_path.as_bytes()).collect();
                format!("{}/projects/{}/merge_requests", self.get_base_url(config), encoded_target)
            },
            Platform::Gitea |
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo => format!("{}/repos/{}/{}/pulls", self.get_base_url(config), options.target.owner, options.target.repo),
        }
    }
}

