use crate::structs::User;
use crate::common::traits::Convert;


impl Convert for super::User {
    type Local = User;

    fn convert(&self) -> Self::Local {
        User {
            id: self.login.clone(),
            name: self.login.to_owned(),
            path: None,
            url: self.html_url.to_owned(),
        }
    }
}
