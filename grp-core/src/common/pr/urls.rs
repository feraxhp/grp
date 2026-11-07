use urlencoding::encode;

use crate::structs::PullRequestOptions;
use crate::Platform;
use crate::Config;


impl Platform {
    pub(crate) fn url_create_pr(&self, config: &Config, options: &PullRequestOptions) -> String {
        match self {
            Platform::Gitlab => {
                let origin_path = format!("{}/{}", options.origin.owner, options.origin.repo);
                let encoded_origin = encode(&origin_path);
                format!("{}/projects/{}/merge_requests", self.get_base_url(config), encoded_origin)
            },
            Platform::Gitea |
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo => format!("{}/repos/{}/{}/pulls", self.get_base_url(config), options.target.owner, options.target.repo),
        }
    }
}
