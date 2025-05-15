// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use futures::future::join_all;
use hyper::header::HeaderValue;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response, Url};

pub async fn paggination_mannager(
    url: String,
    header_map: HeaderMap
) -> (Vec<Response>, Vec<Error>){

    let mut responses: Vec<Response> = Vec::new();
    let mut errors: Vec<Error> = Vec::new();

    let client = Client::new();

    let mut url = Url::parse(&url).unwrap();
    url.query_pairs_mut()
        .append_pair("per_page", "100")
        .append_pair("page", "1");

    let response = match client.get(url).headers(header_map.clone()).send().await {
        Ok(response) => response,
        Err(e) => {
            errors.push(Error::new(ErrorType::Unknown, vec![e.to_string().as_str()]));
            return (responses, errors);
        }
    };

    let headers = response.headers().clone();
    responses.push(response);

    let next_links = extract_links(headers.get("link"));
    if next_links.is_empty() {
        return (responses, errors);
    }

    let futures: Vec<_> = next_links.iter().map(|link| {
        client.get(link).headers(header_map.clone()).send()
    }).collect();

    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok(response) => responses.push(response),
            Err(e) => errors.push(Error::new(ErrorType::Unknown, vec![e.to_string().as_str()])),
        }
    }

    (responses, errors)
}

fn extract_links(link_header: Option<&HeaderValue>) -> Vec<String> {
    let headers = link_header.clone();
    match link_header {
        None => { Vec::new() }
        Some(_) => {
            let links = headers.unwrap().to_str().unwrap().split(',').collect::<Vec<&str>>();

            links.iter().map(|link| {
                let parts: Vec<&str> = link.split(';').collect();

                if parts.len() == 2 {
                    let url = parts[0].trim().trim_start_matches('<').trim_end_matches('>');
                    let label = parts[1].trim().trim_start_matches('<').trim_end_matches('>');

                    match label {
                        "rel=\"next\"" => url.to_string(),
                        _ => "".to_string()
                    }
                } else { "".to_string() }
            }).collect::<Vec<String>>().into_iter().filter(|link| link != "").collect()
        }
    }
}