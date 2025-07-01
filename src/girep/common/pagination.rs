use std::str::FromStr;

use futures::future::join_all;
use hyper::header::HeaderValue;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response, Url};
use crate::girep::error::structs::Error;
use crate::girep::error::types::ErrorType;

pub async fn pagination(
    url: String,
    headers: HeaderMap
) -> (Vec<Response>, Option<Error>){

    let mut responses: Vec<Response> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let client = Client::new();

    let mut url = Url::parse(&url).unwrap();
    url.query_pairs_mut()
        .append_pair("per_page", "10")
        .append_pair("page", "1");

    let response = match client.get(url).headers(headers.clone()).send().await {
        Ok(response) => response,
        Err(e) => {
            errors.push(e.to_string());
            return (responses, Some(Error::new(ErrorType::PaginationErrors, errors)));
        }
    };

    let mut response_headers = response.headers().clone();
    responses.push(response);
    
    let mut errors = Vec::new();
    let mut next = true;
    
    while next {
        let (next_links, last)  = extract_links(response_headers.get("link"));
        
        if next_links.is_empty() {
            let error = if errors.is_empty() { None }
            else { Some(Error::new(ErrorType::PaginationErrors, errors)) };
            return (responses, error);
        }
        
        let futures: Vec<_> = next_links.iter().map(|link| {
            client.get(link.as_str()).headers(headers.clone()).send()
        }).collect();
    
        let results = join_all(futures).await;
    
        for result in results {
            match result {
                Ok(response) => {
                    response_headers = response.headers().clone();
                    responses.push(response)
                },
                Err(e) => errors.push(e.to_string()),
            }
        }
        
        next = !last;
    }

    let error = if errors.is_empty() { None }
    else { Some(Error::new(ErrorType::PaginationErrors, errors)) };

    (responses, error)
}

fn extract_links(link_header: Option<&HeaderValue>) -> (Vec<Url>, bool) {
    let mut links = Vec::new();

    let header = match link_header.and_then(|header| header.to_str().ok()) {
        Some(h) => h,
        None => return (links, true),
    };
    
    for parts in header.split(',') {
        let part = parts.split(';').collect::<Vec<&str>>();
        
        let url = match part.get(0) {
            Some(url) => match Url::from_str(url.trim().trim_matches('<').trim_matches('>')) {
                Ok(url) => url, Err(_) => continue,
            },
            None => continue,
        };
        
        let rel = match part.get(1) {
            Some(rel) => rel.trim(),
            None => continue,
        };
        
        if rel == "rel=\"last\"" { 
            links.clear();
            let mut last = 0;
            let pairs = url.query_pairs();
            for (key, value) in pairs {
                if key == "page" {
                    last = value.parse::<u8>().unwrap_or(0);
                }
            }
            if last > 1 {
                for i in 2..=last {
                    let mut new_url = url.clone();
                    new_url.query_pairs_mut()
                        .clear()
                        .extend_pairs(url.query_pairs().filter(|(k, _)| k != "page"))
                        .append_pair("page", &i.to_string());
                    links.push(new_url);
                }
            }
            return (links, true);
        } else if rel == "rel=\"next\"" { 
            links.push(url);
        } else { continue; }
        
    }
    
    return (links, false);
}