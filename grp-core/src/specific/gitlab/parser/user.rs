use crate::structs::User;
use crate::common::traits::Convert;


impl Convert for super::User {
    type Local = User;

    fn convert(&self) -> Self::Local {
        User {
            id: self.id.to_string(),
            name: self.username.to_owned(),
            path: None,
            url: self.web_url.to_owned(),
        }
    }
}
