use reqwest::header::HeaderMap;

pub fn get_auth_header(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "reqwest > rust > grp".parse().unwrap());
    headers.insert("content-type.rs", "application/json".parse().unwrap());
    headers.insert("authorization", format!("Bearer {}", token).parse().unwrap());

    headers
}