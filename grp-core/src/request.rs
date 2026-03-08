use async_stream::stream;
use futures::Stream;
use hyper::header::HeaderValue;
use reqwest::{Client, IntoUrl, RequestBuilder, Response, Url};
use serde::Serialize;

use crate::error::errors::parsing::Parsing;
use crate::error::errors::request::Request;
use crate::structs::Context;

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
    
    pub fn pagginate(&self, 
        url: String, 
        config: &Config, 
        context: Context
    ) -> impl Stream<Item = Result<String, Error>> { stream! {
        let client = Client::new();
        let headers = self.get_auth_header(&config.token);
    
        let size = 10;
        let mut url = Url::parse(&url).map_err(|e| Parsing::url(e, &url) )?;
        
        url.query_pairs_mut().append_pair("page", "1").append_pair("per_page", &size.to_string());
        
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
            
            yield Ok(string);
            
            next = extract_next(response_headers.get("link"))?;
        }
    }}
}

fn extract_next(link_header: Option<&HeaderValue>) -> Result<Option<Url>, Error> {
    let header = match link_header.and_then(|header| header.to_str().ok()) {
        Some(h) => h,
        None => return Ok(None),
    };
    
    let url = header.split(",")
        .find_map(|link| -> Option<_> {
            if link.ends_with(r#"; rel="next""#) {
                Some(link.trim().trim_matches('<').replace(r#">; rel="next""#, ""))
            }
            else { None }
        });
    
    if url.is_none() { return Ok(None) };
    let url = url.unwrap();
    Url::parse(&url)
        .map(|e| Some(e))
        .map_err(|e| Parsing::url(e, &url) )
}