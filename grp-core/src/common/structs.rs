/// # Repo (repository)
/// 
/// Represents a repository for any platform 
/// it contains varios properties that are shared 
/// across all the repositories.
/// 
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
pub struct Context {
    pub request_type: RequestType,
    pub owner: Option<String>,
    pub repo: Option<String>,
    pub additional: Option<String>,
}

/// # RequestType
/// 
/// an enum used to represent the aim of the request.
///  
#[derive(Clone)]
pub enum RequestType {
    List,
    Create,
    Delete,
    DeletePermanent,
    UserList,
    ListOrg,
    CreateOrg,
    DeleteOrg,
    RepositoryDetails
}
