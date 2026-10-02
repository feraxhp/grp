use chrono::Utc;
use chrono::DateTime;
use reqwest::Url;

#[derive(Clone, Debug)]
pub struct Pager {
    pub next: Option<Url>,
    pub last: Option<Url>,
}

#[derive(Clone, Debug)]
pub struct PaginatorResult<T> {
    pub result: T,
    pub pager: Pager
}


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
/// 3. `path`: an optional path for the group or organization (Gitlab)
#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub path: Option<String>, // Optional path for the group, for Gitlab
    pub url: String,
}

// # Repo (repository)
/// 
/// Represents a repository for any platform 
/// it contains varios properties that are shared 
/// across all the repositories.
/// 
#[derive(Clone, Debug)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub private: Option<bool>,
    pub url: String,
    pub git: String,
    pub description: Option<String>,
}

/// # Contex
/// 
/// This object allows to share more debug informacion for 
/// the error, if some platform fails.
/// 
#[derive(Clone, Debug)]
pub struct Context {
    pub request_type: RequestType,
    pub owner: Option<String>,
    pub repo: Option<String>,
    pub additional: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Issue {
    pub author: User,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub url: String,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct Comment {
    pub id: u64,
    pub author: User,
    pub body: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// # RequestType
/// 
/// an enum used to represent the aim of the request.
///  
#[derive(Clone, Debug)]
pub enum RequestType {
    List,
    Create,
    Delete,
    DeletePermanent,
    UserList,
    ListOrg,
    CreateOrg,
    DeleteOrg,
    RepositoryDetails,
    ListIssues,
    ListIssuesComments,
}
