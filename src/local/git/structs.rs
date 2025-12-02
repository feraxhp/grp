
pub struct GitUtils;

#[allow(dead_code)]
pub enum Action {
    Push,
    Pull,
    Fetch,
    /// platform
    Clone(String),
    /// Name, Url
    SetRemote(String, String),
}