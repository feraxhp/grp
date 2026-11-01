use std::collections::HashSet;
use std::fmt::Display;

use futures::{Stream, StreamExt};
use mynox::Error;

use crate::specific::forgejo::parser::Issue;
use crate::structs::PaginatorResult;
use crate::animation::Animation;
use crate::structs::RequestType;
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
    
    fn destination(&self, platform: &Platform, config: &Config) -> (String, RequestType) {
        (
            format!("{}/repos/issues/search?state=open&type=issues&{}", platform.get_base_url(config), self.filter()),
            RequestType::ListIssues,
        )
    }
}

pub async fn list_issues_by_scope<A, const N: usize>(
    platform: &Platform,
    scopes: [Scope; N],
    config: &Config,
    animation: &Box<A>,
    page: u64,
) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
where 
    A: Animation + ?Sized,
{
    let destinations = scopes.map(|scope| scope.destination(platform, config));
    let mut seen = HashSet::new();
    
    animation.change_message("requesting issues...");
    Ok(
        platform
            .pagginates(destinations, config, page)
            .map(move |response| parse(response, &mut seen))
    )
}

pub(crate) fn parse<const N: usize>(
    response: Result<PaginatorResult<[Option<String>; N]>, Error>,
    seen: &mut HashSet<u64>,
) -> Result<PaginatorResult<Vec<Issue>>, Error> {
    let result = response?;
    let mut todos = Vec::new();
    
    for json in result.result.iter().flatten() {
        let issues: Vec<Issue> = JSON::from_str(json)?;
        todos.extend(issues.into_iter().filter(|issue| seen.insert(issue.id)));
    }
    
    Ok(result.parse_result(todos))
}
