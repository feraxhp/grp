use crate::structs::Comment;
use crate::common::traits::Convert;


impl Convert for super::Comment {
    type Local = Comment;

    fn convert(&self) -> Self::Local {
        Self::Local {
            id: self.id.to_owned(),
            author: self.author.convert(),
            body: self.body.to_owned(),
            url: String::default(),
            created_at: self.created_at.to_owned(),
            updated_at: self.updated_at.to_owned(),
        }
    }
}
