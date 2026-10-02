use crate::structs::User;
use crate::structs::UserType;


impl UserType {
    pub fn get_user(&self) -> User {
        match self {
            UserType::LoggedUser(name_or_id) => name_or_id,
            UserType::LoggedOrg(name_or_id) => name_or_id,
            UserType::UnloggedUser(name_or_id) => name_or_id,
            UserType::UnloggedOrg(name_or_id) => name_or_id,
        }.clone()
    }
}
