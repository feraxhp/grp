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
            RequestType::ListRepo { owner } => write!(f, "listing repos for `{owner}`"),
            RequestType::CreateRepo { owner, repo } => write!(f, "creating `{owner}/{repo}`"),
            RequestType::DeleteRepo { permanent, owner, repo } => {
                if *permanent { write!(f, "-_permanently_-")?; }
                write!(f, " -deleting- `{owner}/{repo}`")?;
                Ok(())
            },
            RequestType::RepositoryDetails { owner, repo } => {
                write!(f, "getting details for `{owner}/{repo}`")?;
                Ok(())
            },
            RequestType::GetLoggedUser => {
                write!(f, "getting the logged user")?;
                Ok(())
            },
            RequestType::ListOrg { way } => {
                write!(f, "Listing orgs {way}")?;
                Ok(())
            },
            RequestType::CreateOrg { name } => write!(f, "creating `{name}`"),
            RequestType::DeleteOrg { permanent, name } => {
                if *permanent { write!(f, "-_permanently_-")?; }
                write!(f, " -deleting- `{name}`")?;
                Ok(())
            },
            RequestType::ListIssues => write!(f, "listing issues"),
            RequestType::IssueDetails { owner, repo, number } => write!(f, "getting details for `{owner}/{repo}#{number}`"),
            RequestType::ListIssuesComments { owner, repo, number } => write!(f, "listing comments for `{owner}/{repo}#{number}`"),
            RequestType::ListTodos => write!(f, "listing todos"),
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
        }
    }
}
