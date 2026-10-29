pub mod issue;
pub mod org;
pub mod repo;
pub mod user;

use std::ops::Deref;
use serde_json::{Value, json};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::{HeaderExactMatcher, body_json, method, path}};


#[derive(Debug)]
pub struct Endpoint<const N:usize > {
    pub headers: [HeaderExactMatcher; N],
    pub payload: Option<Value>,
    pub method: &'static str,
    pub path: &'static str,
    pub status: u16,
    pub body: Value,
    pub query: Option<(&'static str, &'static str)>,
    pub calls: u64,
}

impl<const N: usize> Endpoint<N> {
    pub fn query(mut self, key: &'static str, val: &'static str) -> Self {
        self.query = Some((key, val));
        self
    }
    
    #[allow(dead_code)]
    pub fn calls(mut self, n: u64) -> Self {
        self.calls = n;
        self
    }
}


#[derive(Debug)]
pub struct  Server(MockServer);

impl Deref for Server {
    type Target = MockServer;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Drop for Server {
    fn drop(&mut self) {
        futures::executor::block_on(self.0.reset());
    }
}

impl Server {
    async fn start() -> Self {
        let mock_server = MockServer::start().await;
        Self(mock_server)
    }
    
    async fn with<const N: usize>(&self, value: Endpoint<N>) {
        let mut mock = Mock::given(method(value.method)).and(path(value.path));
        for header in value.headers { mock = mock.and(header); }
        if let Some(payload) = value.payload { mock = mock.and(body_json(payload)) }
        if let Some((k, v)) = value.query { mock = mock.and(wiremock::matchers::query_param(k, v)); }
        
        let mut response = ResponseTemplate::new(value.status);
        if value.status != 204 {
            response = response.set_body_json(value.body);
        }
        mock.respond_with(response).expect(value.calls).mount(self).await
    }
    
    pub async fn verify(&self) {
        self.0.verify().await;
        self.0.reset().await;
    }
}

#[macro_export]
macro_rules! get  { 
    ($status:literal, $path:literal, $body:expr $(, $header:literal: $value:literal )* $(,)?) => { 
        Endpoint { path: $path, payload: None, method: "GET", status: $status, body: $body, headers: [ $( wiremock::matchers::header($header, $value), )* ], query: None, calls: 1 } 
    }; 
    ($status:literal, $path:literal, $body:expr, $headers:expr) => { 
        Endpoint { path: $path, payload: None, method: "GET", status: $status, body: $body, headers: $headers, query: None, calls: 1 } 
    }; 
}

#[macro_export]
macro_rules! post { 
    ($status:literal, $path:literal, $payload:expr, $body:expr $(, $header:literal: $value:literal )* $(,)?) => { 
        Endpoint { path: $path, payload: Some($payload), method: "POST", status: $status, body: $body, headers: [ $( wiremock::matchers::header($header, $value), )* ], query: None, calls: 1 } 
    }; 
    ($status:literal, $path:literal, $payload:expr, $body:expr, $headers:expr) => { 
        Endpoint { path: $path, payload: Some($payload), method: "POST", status: $status, body: $body, headers: $headers, query: None, calls: 1 } 
    }; 
}

#[macro_export]
macro_rules! delete { 
    ($status:literal, $path:literal $(, $header:literal: $value:literal )* $(,)?) => { 
        Endpoint { path: $path, payload: None, method: "DELETE", status: $status, body: serde_json::json!({}), headers: [ $( wiremock::matchers::header($header, $value), )* ], query: None, calls: 1 } 
    }; 
    ($status:literal, $path:literal, $headers:expr) => { 
        Endpoint { path: $path, payload: None, method: "DELETE", status: $status, body: serde_json::json!({}), headers: $headers, query: None, calls: 1 } 
    }; 
}

#[macro_export]
macro_rules! server {
    ( $( $request:expr ),* $(,)?) => {(async {
        let server = Server::start().await;
        $( server.with($request).await; )*
        server
    }).await };
}


#[macro_export]
macro_rules! value { ($file:literal) => { {
    let str = include_str!($file);
    let json: Value = serde_json::from_str(str).unwrap();
    json
}}; (array $file:literal) => { {
    let str = format!("[{}]", include_str!($file));
    let json: Value = serde_json::from_str(&str).unwrap();
    json
}};}

#[macro_export]
macro_rules! faild { ($errors:expr) => { 
    let error = $errors.into_iter().collect::<mynox::OptionError>();
    
    if let mynox::OptionError::Some(e) = error {
        return Err(e.into())
    }
}; }

#[macro_export]
macro_rules! auth { 
    (gh) => { [
        wiremock::matchers::header("User-Agent", "girep-feraxhp"),
        wiremock::matchers::header("Accept", "application/vnd.github+json"),
        wiremock::matchers::header("X-GitHub-Api-Version", "2022-11-28"),
        wiremock::matchers::header("Authorization", "Bearer my-token")
    ] }; 
    (fj) => { [
        wiremock::matchers::header("User-Agent", "reqwest > rust > grp"),
        wiremock::matchers::header("authorization", "Bearer my-token"),
    ] }; 
    (gl) => { [
        wiremock::matchers::header("User-Agent", "reqwest > rust > grp"),
        wiremock::matchers::header("authorization", "Bearer my-token"),
    ] }; 
}

#[macro_export]
macro_rules! config { 
    (gh $server:expr) => { Config::new_unsafe("internal", "octocat", "my-token", $server.address().to_string()) }; 
    (fj $server:expr) => { Config::new_unsafe("internal", "forgejo_user", "my-token", $server.address().to_string()) }; 
    (gl $server:expr) => { Config::new_unsafe("internal", "gitlab_user", "my-token", $server.address().to_string()) }; 
}

#[tokio::test]
#[allow(unused)]
async fn makro() {
    let get = get!(200, "/path", json!({}), "Authorization": "Bearer my-token");
    let post = post!(200, "/path", json!({}), json!({}), "Authorization": "Bearer my-token");
}
