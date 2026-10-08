use std::fmt::Display;

use futures::{Stream, StreamExt};

use crate::Config;
use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::animation::Animation;
use crate::specific::github::parser::Issue;
use crate::structs::Context;
use crate::structs::PaginatorResult;
use crate::structs::RequestType;


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
) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Issue>>, Error>>, Error> 
where 
    A: Animation + ?Sized,
{
    let url = format!("{}/issues?state=open&{}", platform.get_base_url(&config.endpoint), scope.filter());

    let context = Context {
        request_type: RequestType::List,
        owner: None,
        repo: None,
        additional: None,
    };

    animation.change_message(format!("requesting {} issues...", scope));
    Ok(
        platform
            .pagginate(url, config, context, 1)
            .map(parse)
    )
}

pub(crate) fn parse(response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Issue>>, Error> {
    let result = response?;
    let todos: Vec<Issue> = JSON::from_str(&result.result)?;
    
    Ok(result.parse_result(todos))
}
