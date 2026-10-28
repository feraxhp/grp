use crate::common::traits::Convert;
use crate::structs::User;


impl Convert for super::User {
    type Local = User;

    fn convert(&self) -> Self::Local {
        User {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            path: None,
        }
    }
}
