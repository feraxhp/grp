

pub enum ErrorType {
    /// vec: [>=2]
    /// - 0 : path
    /// - 1 : object (dir, file, etc.)
    /// - 2+: additional information
    Path404,
    /// vec: [>=2]
    /// - 0 : object
    /// - 1 : message
    /// - 2+: additional information
    Obj404,
    /// vec: [>=2]
    /// - 0 : error message
    /// - 1 : config file path
    /// - 2+: additional information
    UsettingsParsing,
    /// vec: [>=2]
    /// - 0 : description
    /// - 1 : text
    /// - 2+: additional information
    ResponseParsing,
    /// vec: [>=1]
    /// - 0+: error message
    PaginationErrors,
    /// vec: [>=1]
    /// - 0+: error message
    Unknown,
    /// vec: [==2]
    /// - 0 : pconf
    /// - 1 : user
    Unauthorized,
    /// vec: [==1]
    ///  - 0 : file-path
    Incomplete,
    /// vec: [>=2]
    /// - 0 : pconf
    /// - 1+: scopes
    BadTokenScope,
    /// vec: [>=1]
    /// - 0 : user
    /// - 1+: additional information
    NotOwnerFound,
    /// vec: [>=1]
    /// - 0 : org
    /// - 1+: additional information
    NotOrganizationFound,
    /// vec: [==2]
    /// - 0 : owner
    /// - 1 : repo
    NotRepoFound,
    /// vec: [>=2]
    /// - 0 : type ("Repo", "Orgs", etc.)
    /// - 1 : name ("owner", "name", etc.)
    /// - 2 : additional info (e.g., "repo_name", etc.)
    AlreadyExists,
    /// vec: [==2]
    /// - 0 : message
    /// - 1 : context
    FetchFailed,
    /// vec: [>=2]
    /// - 0 : platform
    /// - 1 : action
    /// - 2+: additional info
    Unsupported,
    /// vec: [==1]
    /// - 0 : path
    LocalRepoNotFound,
}
