use reqwest::header::HeaderMap;

pub fn get_auth_header(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "reqwest > rust > grp".parse().unwrap());
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    headers
}