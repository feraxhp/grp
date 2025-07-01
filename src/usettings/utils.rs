use crate::girep::config::Config;
use crate::usettings::structs::Pconf;

impl Pconf {
    pub fn to_config(&self) -> Config {
        Config::new(
            self.name.clone(),
            self.owner.clone(),
            self.token.clone(),
            self.endpoint.clone(),
        )
    }
}