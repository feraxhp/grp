use std::sync::OnceLock;

use async_stream::stream;
use futures::Stream;

use hyper::header::HeaderValue;

use mynox::Extend;
use mynox::Error;

use reqwest::RequestBuilder;
use reqwest::Response;
use reqwest::Client;
use reqwest::IntoUrl;
use reqwest::Url;

use serde::Serialize;

use crate::helpers::request::RequestError;
use crate::helpers::parsing::ParsingUrl;
use crate::structs::PaginatorResult;
use crate::structs::RequestType;
use crate::structs::Pager;

use super::platform::Platform;
use super::config::Config;


pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(|| { Client::new() })
}


impl Platform {
    async fn send(req: RequestBuilder) -> Result<Response, Error> { req.send().await.fetch("Please check your ethernet conection") }
    pub async fn get<U: IntoUrl>(&self, url: U, auth: bool, config: &Config) -> Result<Response, Error> {
        let client = client();
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
        let client = client();
        
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
        let client = client();
        
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
        request_type: RequestType, 
        page: u64,
    ) -> impl Stream<Item = Result<PaginatorResult<String>, Error>> { stream! {
        let client = client();
        let headers = self.get_auth_header(&config.token);
        
        let size = 50;
        let mut url = Url::parse(&url).as_parsing_url_error(&url)?;
        
        url.query_pairs_mut()
            .append_pair("page", &page.to_string())
            .append_pair(self.limit(), &size.to_string());
        
        let mut next = Some(url);
        let mut last = None;
        
        while let Some(url) = next {
            
            let notes = [format!("url: `'{url}'`")];
            
            let response = client.get(url.clone()).headers(headers.clone()).send().await
                    .fetch("Error during paggination")
                    .notes(notes)
            ?;
            
            let response_headers = response.headers().clone();
            let string = self.unwrap(response,  "Faild getting reponse", config, &request_type).await?;
            
            let mut pagger = extract_next(response_headers.get("link"))?;
            if pagger.last.is_some() { last = pagger.last.clone() }
            else if last.is_some() { pagger.last = last.clone() }
            
            next = pagger.next.clone();
            
            yield Ok(PaginatorResult { result: string, pager: pagger });
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
    let url = Url::parse(&url).map(Some)
            .as_parsing_url_error(&url)?;

    match last_url {
        Some(last_url) => {
            let last_url = Url::parse(&last_url).map(Some)
                .as_parsing_url_error(&last_url)?;
            
            Ok(Pager{ next: url, last: last_url })
        },
        None => Ok(Pager{ next: url, last: None }),
    }
}

impl Pager {
    fn none() -> Self { Self { next: None, last: None } }
    
    pub fn last_number(&self) -> Option<u64>{
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
            Some(Some(Ok(number))) => Some(number),
            _ => None,
        }
    }
}
