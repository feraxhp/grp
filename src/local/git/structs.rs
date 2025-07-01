
pub struct GitUtils;

#[allow(dead_code)]
pub enum Action {
    Push,
    Pull,
    /// platform
    Clone(String),
    /// Name, Url
    SetRemote(String, String),
}