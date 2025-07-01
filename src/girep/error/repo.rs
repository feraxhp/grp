#[macro_export]
macro_rules! repo {
    ($user:expr, $repo:expr) => {{
        let result = format!("{}/{}", $user, $repo);
        result.trim_matches('/').to_string()
    }};
}