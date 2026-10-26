use std::fmt::Display;

use futures::{Stream, StreamExt};
use mynox::Error;

use crate::specific::forgejo::parser::Issue;
use crate::structs::PaginatorResult;
use crate::animation::Animation;
use crate::structs::RequestType;
use crate::structs::Context;
use crate::Platform;
use crate::Config;
use crate::JSON;


pub enum Scope {
    CreatedByMe,
    AssignedToMe,
    Mentioned,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scope::CreatedByMe => write!(f, "created_by_me"),
            Scope::AssignedToMe => write!(f, "assigned_to_me"),
            Scope::Mentioned => write!(f, "mentioned"),
        }
    }
}

impl Scope {
    fn filter(&self) -> &'static str {
        match self {
            Scope::CreatedByMe => "created=true",
            Scope::AssignedToMe => "assigned=true",
            Scope::Mentioned => "mentioned=true",
        }
    }
}

pub async fn list_issues_by_scope<A>(
    platform: &Platform,
    scope: Scope,
    config: &Config,
    animation: &Box<A>,
) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
where 
    A: Animation + ?Sized,
{
    let url = format!("{}/repos/issues/search?state=open&type=issues&{}", platform.get_base_url(config), scope.filter());
    animation.change_message(format!("requesting {} issues...", scope));
    Ok(
        platform
            .pagginate(url, config, RequestType::ListIssues, 1)
            .map(parse)
    )
}

pub(crate) fn parse(response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Issue>>, Error> {
    let result = response?;
    let todos: Vec<Issue> = JSON::from_str(&result.result)?;
    
    Ok(result.parse_result(todos))
}
