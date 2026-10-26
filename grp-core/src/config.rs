use std::fmt::Debug;

/// # Config
/// is the configuration needed to interact with any platform 
/// it contains 4 atributes
/// 
/// 1. `pconf`: the name for the _configuration_, showed in the error messages.
/// 2. `user`: the _user_ or _org-name_ registerd in the **platform**.
/// 3. `token`: the token to _authenticate_ the user.
/// 4. `endpont`: the _endpoint_ used to interact with the **platform**.
/// 
/// # Example
/// ~~~
/// use grp_core::Config;
/// 
/// let config = Config::new("internal", "feraxhp", "gh-******", "api.github.com");
/// ~~~
/// 
#[derive(Clone)]
pub struct Config {
    pub(crate) unsafed: bool,
    pub pconf: String,
    pub user: String,
    pub token: String,
    pub endpoint: String,
}

impl Config {
    pub fn new<P, U, T, E>(
        pconf: P, user: U,
        token: T, endpoint: E
    ) -> Self
    where 
        P: Into<String>,
        U: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        Config {
            unsafed: false,
            pconf: pconf.into(),
            user: user.into(),
            token: token.into(),
            endpoint: endpoint.into()
        }
    }
    
    pub fn new_unsafe<P, U, T, E>(
        pconf: P, user: U,
        token: T, endpoint: E
    ) -> Self 
    where 
        P: Into<String>,
        U: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        Config {
            unsafed: true,
            pconf: pconf.into(),
            user: user.into(),
            token: token.into(),
            endpoint: endpoint.into()
        }
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("unsafed", &self.unsafed)
            .field("pconf", &self.pconf)
            .field("user", &self.user)
            .field("token", &"[REDACTED]")
            .field("endpoint", &self.endpoint)
            .finish()
    }
}