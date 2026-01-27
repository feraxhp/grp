
/// # UserType
/// represents the type of the user that was given.
/// 
/// 1. `LoggedUser`: **User** that is logged in
/// 2. `LoggedOrg`: **Organization** that belongs to the logged user
/// 3. `UnloggedUser`: **User** that is not logged in
/// 4. `UnloggedOrg`: **Organization** that does not belong to the logged user
#[derive(Clone, Debug)]
pub enum UserType {
    LoggedUser(User),
    LoggedOrg(User),
    UnloggedUser(User),
    UnloggedOrg(User),
}

/// # User
/// Represents a _user_ or _org_ for every platform.
/// 
/// 1. `id`: the id of the user.
/// 2. `name`: the name of the user.
/// 3. `path`: an optional path for the group or organization (gitea)
#[derive(Clone, Debug)]
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
