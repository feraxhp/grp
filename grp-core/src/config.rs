

#[derive(Clone)]
pub struct Config {
    pub pconf: String,
    pub user: String,
    pub token: String,
    pub endpoint: String,
}

impl Config {
    pub fn new(
        pconf: String,
        user: String,
        token: String,
        endpoint: String
    ) -> Self {
        Config {
            pconf,
            user,
            token,
            endpoint
        }
    }
}
