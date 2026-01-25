#[allow(dead_code)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub private: Option<bool>,
    pub url: String,
    pub git: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
pub struct Context {
    pub request_type: RequestType,
    pub owner: Option<String>,
    pub repo: Option<String>,
    pub additional: Option<String>,
}

#[allow(dead_code)]
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
