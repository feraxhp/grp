use std::fmt::Display;

use futures::{Stream, StreamExt};
use mynox::Error;

use crate::structs::PaginatorResult;
use crate::animation::Animation;
use crate::structs::RequestType;
use crate::Platform;
use crate::Config;
use crate::JSON;


#[allow(unused)]
pub enum Filter {
    All,
    Created,
    Assigned,
    Mentioned,
    Subscribed,
    Repos,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::All => write!(f, "all"),
            Filter::Created => write!(f, "created"),
            Filter::Assigned => write!(f, "assigned"),
            Filter::Mentioned => write!(f, "mentioned"),
            Filter::Subscribed => write!(f, "subscribed"),
            Filter::Repos => write!(f, "repos"),
        }
    }
}

impl Filter {
    fn filter(&self) -> &'static str {
        match self {
            Filter::All => "filter=all",
            Filter::Created => "filter=created",
            Filter::Assigned => "filter=assigned",
            Filter::Mentioned => "filter=mentioned",
            Filter::Subscribed => "filter=subscribed",
            Filter::Repos => "filter=repos",
        }
    }
}

pub async fn list_issues_by_scope<A>(
    platform: &Platform,
    scope: Filter,
    config: &Config,
    animation: &Box<A>,
    page: usize,
) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<crate::specific::github::parser::Issue>>, Error>>, Error> 
where 
    A: Animation + ?Sized,
{
    let url = format!("{}/issues?state=open&{}", platform.get_base_url(config), scope.filter());

    animation.change_message(format!("requesting {} issues...", scope));
    Ok(
        platform
            .pagginate(url, config, RequestType::ListIssues, page)
            .map(parse)
    )
}

pub(crate) fn parse(response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<crate::specific::github::parser::Issue>>, Error> {
    let result = response?;
    let todos: Vec<crate::specific::github::parser::Issue> = JSON::from_str(&result.result)?;
    
    Ok(result.parse_result(todos))
}
