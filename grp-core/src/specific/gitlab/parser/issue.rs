use crate::structs::Issue;
use crate::common::traits::Convert;


impl Convert for super::Issue {
    type Local = Issue;

    fn convert(&self) -> Self::Local {
        Self::Local {
            author: self.author.convert(),
            number: self.iid.to_owned(),
            title: self.title.to_owned(),
            body: self.description.to_owned(),
            state: self.state.to_owned(),
            url: self.web_url.to_owned(),
            locked: false,
            created_at: self.created_at.to_owned(),
            updated_at: self.updated_at.to_owned(),
        }
    }
}
