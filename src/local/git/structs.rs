
pub struct GitUtils;

pub enum Action {
    Push,
    Pull,
    Fetch,
    /// platform
    Clone,
    /// Name, Url
    SetRemote(String, String),
}

