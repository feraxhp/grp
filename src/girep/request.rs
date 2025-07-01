use reqwest::{Client, IntoUrl, RequestBuilder, Response};
use serde::Serialize;

use crate::girep::config::Config;
use crate::girep::error::types::ErrorType;
use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;

impl Platform {
    async fn send(req: RequestBuilder) -> Result<Response, Error> {
        req.send().await.map_err( |e| Error::new(
            ErrorType::FetchFailed, 
            vec![e.to_string(), "Please check you ethernet conection".to_string()]
        ))
    }
    pub async fn get<U: IntoUrl>(&self, url: U, header: bool, config: &Config) -> Result<Response, Error> {
        let client = Client::new();
        
        let mut req = client.get(url);
        if header {
            req = req.headers(self.get_auth_header(&config.token));
        };
        
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
}