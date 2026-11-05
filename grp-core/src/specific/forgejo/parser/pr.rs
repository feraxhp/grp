use crate::structs::PullRequest;
use crate::common::traits::Convert;


impl Convert for super::Pr {
    type Local = PullRequest;

    fn convert(&self) -> Self::Local {
        PullRequest {
            id: self.id.to_owned(),
            author: self.user.convert(),
            title: self.title.to_owned(),
            body: self.body.to_owned(),
            url: self.html_url.to_owned(),
            number: self.number.to_owned(),
            created_at: self.created_at.to_owned(),
            updated_at: self.updated_at.to_owned(),
        }
    }
}
