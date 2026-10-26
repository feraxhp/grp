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
                write!(f, "Deleting")?;
                if *permanent { write!(f, " -permanent-")?; }
                write!(f, " `{owner}/{repo}`")?;
                Ok(())
            },
            _ => todo!(),
        }
    }
}

impl AsRef<str> for RequestType {
    fn as_ref(&self) -> &str {
        match self {
            RequestType::ListRepo { owner: _ } => "list_repo",
            RequestType::CreateRepo { owner: _, repo: _ } => "create_repo",
            RequestType::DeleteRepo { permanent: _, owner: _, repo: _ } => "delete_repo",
            _ => todo!(),
        }
    }
}
