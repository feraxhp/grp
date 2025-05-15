use reqwest::header::HeaderMap;

pub fn get_auth_header(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    headers
}