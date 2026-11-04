use std::fmt::Display;

use futures::future::Ready;

use mynox::Error;
use crate::structs::{PaginatorResult, RequestType};


pub fn skip_empty<T>(res: &Result<PaginatorResult<Vec<T>>, Error>) -> Ready<bool> {
    let keep = match res {
        Ok(batch) => !batch.result.is_empty(),
        Err(_) => true,
    };
    
    futures::future::ready(keep)
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestType::ListRepo { owner } => write!(f, "listing repos for `<raw>{owner}</raw>`"),
            RequestType::CreateRepo { owner, repo } => write!(f, "creating `<raw>{owner}/{repo}</raw>`"),
            RequestType::DeleteRepo { permanent, owner, repo } => {
                if *permanent { write!(f, "-_permanently_-")?; }
                write!(f, " -deleting- `<raw>{owner}/{repo}</raw>`")?;
                Ok(())
            },
            RequestType::RepositoryDetails { owner, repo } => {
                write!(f, "getting details for `<raw>{owner}/{repo}</raw>`")?;
                Ok(())
            },
            RequestType::GetLoggedUser => {
                write!(f, "getting the logged user")?;
                Ok(())
            },
            RequestType::ListOrg { way } => {
                write!(f, "Listing orgs {way}</raw>")?;
                Ok(())
            },
            RequestType::CreateOrg { name } => write!(f, "creating `<raw>{name}</raw>`"),
            RequestType::DeleteOrg { permanent, name } => {
                if *permanent { write!(f, "-_permanently_-")?; }
                write!(f, " -deleting- `<raw>{name}</raw>`")?;
                Ok(())
            },
            RequestType::ListIssues => write!(f, "listing issues"),
            RequestType::IssueDetails { owner, repo, number } => write!(f, "getting details for `<raw>{owner}/{repo}#{number}</raw>`"),
            RequestType::ListIssuesComments { owner, repo, number } => write!(f, "listing comments for `<raw>{owner}/{repo}#{number}</raw>`"),
            RequestType::ListTodos => write!(f, "listing todos"),
            RequestType::CreatePr(pull_request_options) => {
                write!(f, "creating pr: `<raw>{}/{}:</raw>+<raw>{}</raw>+` -> `<raw>{}/{}:</raw>+<raw>{}</raw>+`", 
                    &pull_request_options.origin.owner,
                    &pull_request_options.origin.repo,
                    &pull_request_options.origin.branch,
                    &pull_request_options.target.owner,
                    &pull_request_options.target.repo,
                    &pull_request_options.target.branch,
                )
            },
        }
    }
}

impl AsRef<str> for RequestType {
    fn as_ref(&self) -> &str {
        match self {
            RequestType::ListRepo { owner: _ } => "list_repo",
            RequestType::CreateRepo { owner: _, repo: _ } => "create_repo",
            RequestType::DeleteRepo { permanent: _, owner: _, repo: _ } => "delete_repo",
            RequestType::RepositoryDetails { owner: _, repo: _ } => "repository_details",
            RequestType::GetLoggedUser => "get_logged_user",
            RequestType::ListOrg { way: _ } => "list_org",
            RequestType::CreateOrg { name: _ } => "create_org",
            RequestType::DeleteOrg { permanent: _, name: _ } => "delete_org",
            RequestType::ListIssues => "list_issues",
            RequestType::IssueDetails { owner: _, repo: _, number: _ } => "issue_details",
            RequestType::ListIssuesComments { owner: _, repo: _, number: _ } => "list_issues_comments",
            RequestType::ListTodos => "list_todos",
            RequestType::CreatePr(_) => "create_pull_request",
        }
    }
}
