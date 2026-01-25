
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum UserType {
    LoggedUser(User),    // User that is logged in
    LoggedOrg(User),     // Organization that belongs to the logged user
    UnloggedUser(User),  // User that is not logged in
    UnloggedOrg(User),   // Organization that does not belong to the logged user
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: String,
    pub name: String,
    pub path: Option<String>, // Optional path for the group, for Gitea
}

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
