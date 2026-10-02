use crate::structs::User;
use crate::common::traits::Convert;


impl Convert for super::Group {
    type Local = User;

    fn convert(&self) -> Self::Local {
        User {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            path: Some(self.full_path.to_owned()),
            url: self.web_url.to_owned(),
        }
    }
}
