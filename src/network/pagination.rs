use std::future::Future;
use hyper::HeaderMap;
use reqwest::Response;

pub(crate) async fn pagination_manager(
    url: &str,
    headers: HeaderMap,
    page: Option<u32>,
) -> Vec<Response> {
    let mut responses: Vec<String> = Vec::new();

    let mut url = match page {
        Some(page) => format!("{}?page={}", url, page),
        None => url.to_string(),
    };

    let mut has_next = true;

    while has_next {

        let client = reqwest::Client::new();

        match client.get(url.clone()).headers(headers.clone()).send().await {
            Ok(result) => {
                let headers = result.headers();
                let link = headers.get("link").unwrap_or_default().to_str();
                eprintln!("Link: {}", link);
                let response_text = match error_manager(result) {
                    None => {}
                    Some(text) => {
                        responses.push(text);
                    }
                };
            }
            Err(_) => { }
        }
    }

    responses
}