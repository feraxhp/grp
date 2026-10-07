use std::fmt::Debug;
use std::fmt::Display;

use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;

use crate::Config;
use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::animation::Animation;
use crate::specific::gitlab::parser::Todos;
use crate::structs::Context;
use crate::structs::PaginatorResult;
use crate::structs::RequestType;


pub async fn list_todos<T, A, D>(
    platform: &Platform,
    target: Option<D>, 
    config: &Config,
    animation: &Box<A>,
) -> Result<impl Stream<Item = Result<PaginatorResult<Vec<Todos<T>>>, Error>>, Error> 
where 
    T: Debug + DeserializeOwned,
    A: Animation + ?Sized,
    D: Display,
{
    let url = match target {
        Some(s) => format!("{}/todos?target_type={}", platform.get_base_url(&config.endpoint), s),
        None => format!("{}/todos", platform.get_base_url(&config.endpoint)),
    };
    
    let context = Context {
        request_type: RequestType::List,
        owner: None,
        repo: None,
        additional: None,
    };

    animation.change_message("requesting todo items...");
    Ok(
        platform
            .pagginate(url, config, context, 1)
            .map(|result| {
                parse(result)
            })
    )
}

pub(crate) fn parse<T: Debug + DeserializeOwned>(response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Todos<T>>>, Error> {
    let result = response?;
    let todos: Vec<Todos<T>> = JSON::from_str(&result.result)?;
    
    Ok(result.parse_result(todos))
}