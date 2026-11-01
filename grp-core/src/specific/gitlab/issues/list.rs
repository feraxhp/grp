use std::collections::HashSet;
use std::fmt::Display;

use futures::StreamExt;
use futures::Stream;
use mynox::Error;

use crate::specific::gitlab::parser::Issue;
use crate::specific::gitlab::parser::Todos;
use crate::structs::PaginatorResult;
use crate::animation::Animation;
use crate::structs::RequestType;
use crate::Platform;
use crate::Config;
use crate::JSON;


pub enum Scope {
    CreatedByMe,
    AssignedToMe,
    Todos,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scope::CreatedByMe => write!(f, "created_by_me"),
            Scope::AssignedToMe => write!(f, "assigned_to_me"),
            Scope::Todos => write!(f, "todos"),
        }
    }
}

impl Scope {
    fn destination(&self, platform: &Platform, config: &Config) -> (String, RequestType) {
        match self {
            Scope::CreatedByMe |
            Scope::AssignedToMe => (
                format!("{}/issues?state=opened&scope={}", platform.get_base_url(config), self),
                RequestType::ListIssues,
            ),
            Scope::Todos => (
                format!("{}/todos?target_type=Issues", platform.get_base_url(config)),
                RequestType::ListTodos,
            ),
        }
    }
    
    fn parse(&self, json: &String) -> Result<Vec<Issue>, Error> {
        match self {
            Scope::CreatedByMe |
            Scope::AssignedToMe => JSON::from_str(json),
            Scope::Todos => {
                let todos: Vec<Todos<Issue>> = JSON::from_str(json)?;
                Ok(todos.into_iter().map(|b| b.target).collect())
            },
        }
    }
}

pub async fn list_issues_by_scope<A, const N: usize>(
    platform: &Platform,
    scopes: [Scope; N],
    config: &Config,
    animation: &Box<A>,
) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
where 
    A: Animation + ?Sized,
{
    let destinations = std::array::from_fn(|i| scopes[i].destination(platform, config));
    let mut seen = HashSet::new();

    animation.change_message("requesting issues...");
    Ok(
        platform
            .pagginates(destinations, config, 1)
            .map(move |response| parse(response, &scopes, &mut seen))
    )
}

pub(crate) fn parse<const N: usize>(
    response: Result<PaginatorResult<[Option<String>; N]>, Error>,
    scopes: &[Scope; N],
    seen: &mut HashSet<u64>,
) -> Result<PaginatorResult<Vec<Issue>>, Error> {
    let result = response?;
    let mut todos = Vec::new();
    
    for (i, content) in result.result.iter().enumerate() {
        if let Some(json) = content {
            let issues = scopes[i].parse(json)?;
            todos.extend(issues.into_iter().filter(|issue| seen.insert(issue.id)));
        }
    }
    
    Ok(result.parse_result(todos))
}
