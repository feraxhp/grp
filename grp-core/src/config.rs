
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
    pub pconf: String,
    pub user: String,
    pub token: String,
    pub endpoint: String,
}

impl Config {
    pub fn new<T: Into<String>>(
        pconf: T, user: T,
        token: T, endpoint: T
    ) -> Self {
        Config {
            pconf: pconf.into(),
            user: user.into(),
            token: token.into(),
            endpoint: endpoint.into()
        }
    }
}
