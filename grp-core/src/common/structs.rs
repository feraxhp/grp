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
    pub author: String,
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
    pub author: String,
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
