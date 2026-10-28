use crate::common::traits::Convert;
use crate::structs::Author;


impl Convert for super::Author {
    type Local = Author;

    fn convert(&self) -> Self::Local {
        Author {
            id: self.id.to_string(),
            name: self.username.to_owned(),
            url: self.web_url.to_owned(),
        }
    }
}
