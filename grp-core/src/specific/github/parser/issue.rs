use crate::structs::Issue;
use crate::common::traits::Convert;


impl Convert for super::Issue {
    type Local = Issue;

    fn convert(&self) -> Self::Local {
        Self::Local {
            id: self.id,
            author: self.user.convert(),
            number: self.number.to_owned(),
            title: self.title.to_owned(),
            body: self.body.to_owned(),
            state: self.state.to_owned(),
            url: self.html_url.to_owned(),
            locked: self.locked.to_owned(),
            created_at: self.created_at.to_owned(),
            updated_at: self.updated_at.to_owned(),
        }
    }
}
