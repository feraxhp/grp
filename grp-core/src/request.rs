use async_stream::stream;
use futures::Stream;
use hyper::header::HeaderValue;
use reqwest::{Client, IntoUrl, RequestBuilder, Response, Url};
use serde::Serialize;

use crate::error::errors::parsing::Parsing;
use crate::error::errors::request::Request;
use crate::structs::{Context, PaginatorResult};

use super::config::Config;
use super::platform::Platform;
use super::error::structs::Error;

impl Platform {
    async fn send(req: RequestBuilder) -> Result<Response, Error> {
        req.send().await.map_err( |e| Request::fetch(e, vec!["Please check your ethernet conection"]))
    }
    pub async fn get<U: IntoUrl>(&self, url: U, auth: bool, config: &Config) -> Result<Response, Error> {
        let client = Client::new();
        let mut header = self.get_auth_header(&config.token);
        
        if !auth {
            match self {
                Platform::Github => header.remove("Authorization"),
                Platform::Gitea |
                Platform::Codeberg |
                Platform::Forgejo |
                Platform::Gitlab => header.remove("authorization"),
            };
        }
        
        let req = client.get(url).headers(header);
        
        Platform::send(req).await
    }
    pub async fn post<T, U>(&self, url: U, header: bool, config: &Config, json: &T) -> Result<Response, Error>
    where
        U: IntoUrl,
        T: Serialize + ?Sized,
    {
        let client = Client::new();
        
        let mut request = client
            .post(url)
            .header("content-type", "application/json")
            .json(json);
        
        if header {
            request = request.headers(self.get_auth_header(&config.token));
        }
        
        Platform::send(request).await
    }
    
    pub async fn delete<U: IntoUrl>(&self, url: U, config: &Config) -> Result<Response, Error> {
        let client = Client::new();
        
        let req = client
            .delete(url)
            .headers(self.get_auth_header(&config.token));
        
        Platform::send(req).await
    }
    
    fn limit(&self) -> &'static str {
        match self {
            Platform::Github |
            Platform::Gitlab => "per_page",
            Platform::Gitea |
            Platform::Codeberg |
            Platform::Forgejo => "limit",
        }
    }
    
    pub fn pagginate(&self, 
        url: String, 
        config: &Config, 
        context: Context, 
        page: u64,
    ) -> impl Stream<Item = Result<PaginatorResult<String>, Error>> { stream! {
        let client = Client::new();
        let headers = self.get_auth_header(&config.token);
    
        let size = 50;
        let mut url = Url::parse(&url).map_err(|e| Parsing::url(e, &url) )?;
        
        url.query_pairs_mut()
            .append_pair("page", &page.to_string())
            .append_pair(self.limit(), &size.to_string());
        
        let mut next = Some(url);
        
        while let Some(url) = next {
    
            let response = match client.get(url).headers(headers.clone()).send().await {
                Ok(response) => response,
                Err(e) => {
                    yield Err(Request::fetch("Error during paggination", vec![e]));
                    return;
                }
            };
            
            let response_headers = response.headers().clone();
            let string = match self.unwrap(response,  "Faild getting reponse", &config, context.clone()).await {
                Ok(s) => s,
                Err(e) => {
                    yield Err(e);
                    return;
                },
            };

            let pagger = extract_next(response_headers.get("link"))?;
            next = pagger.next.clone();
            
            yield Ok(PaginatorResult { result: string, last: pagger.last_number() });
        }
    }}
}


fn extract_next(link_header: Option<&HeaderValue>) -> Result<Pager, Error> {
    let header = match link_header.and_then(|header| header.to_str().ok()) {
        Some(h) => h,
        None => return Ok(Pager::none()),
    };
    
    let (next_url, last_url) = header.split(',').fold((None, None), |(next, last), link| {
        let link = link.trim();
        if link.ends_with(r#"; rel="next""#) {
            let url = link
                .trim_start_matches('<')
                .trim_end_matches(r#">; rel="next""#)
                .to_string();
            (Some(url), last)
        } else if link.ends_with(r#"; rel="last""#) {
            let url = link
                .trim_start_matches('<')
                .trim_end_matches(r#">; rel="last""#)
                .to_string();
            (next, Some(url))
        } else {
            (next, last)
        }
    });
    
    if next_url.is_none() { return Ok(Pager::none()) };
    let url = next_url.unwrap();
    let url = Url::parse(&url)
        .map(|e| Some(e))
        .map_err(|e| Parsing::url(e, &url) )?;

    match last_url {
        Some(last_url) => {
            let last_url = Url::parse(&last_url)
                .map(|e| Some(e))
                .map_err(|e| Parsing::url(e, &last_url) )?;
            
            Ok(Pager{ next: url, last: last_url })
        },
        None => Ok(Pager{ next: url, last: None }),
    }
}

pub struct Pager {
    pub next: Option<Url>,
    pub last: Option<Url>,
}

impl Pager {
    fn none() -> Self { Self { next: None, last: None } }
    
    fn last_number(&self) -> Option<u64>{
        let last = self.last.clone().map(|url| {
            let value = url.query_pairs()
                .flat_map(| (key, value) | {
                    if key == "page" { Some(value) } 
                    else { None }
                })
                .last();
            
            value.map(|value| value.parse::<u64>())
        });
        
        match last {
            Some(Some(Ok(number))) => Some(number.clone()),
            _ => None,
        }
    }
}
