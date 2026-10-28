use crate::common::traits::Convert;
use crate::structs::Author;


impl Convert for super::Author {
    type Local = Author;

    fn convert(&self) -> Self::Local {
        Author {
            id: self.login.clone(),
            name: self.login.to_owned(),
            url: self.html_url.to_owned(),
        }
    }
}
