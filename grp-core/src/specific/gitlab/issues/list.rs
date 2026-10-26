use std::fmt::Display;

use futures::StreamExt;
use futures::Stream;
use mynox::Error;

use crate::specific::gitlab::parser::Issue;
use crate::structs::PaginatorResult;
use crate::animation::Animation;
use crate::structs::RequestType;
use crate::Platform;
use crate::Config;
use crate::JSON;


pub enum Scope {
    CreatedByMe,
    AssignedToMe,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scope::CreatedByMe => write!(f, "created_by_me"),
            Scope::AssignedToMe => write!(f, "assigned_to_me"),
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
    let url = format!("{}/issues?state=opened&scope={}", platform.get_base_url(config), scope);

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
