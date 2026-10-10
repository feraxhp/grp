
use grp_core::{Config, Error};

use crate::system::cripto::Cripto;
use super::structs::Pconf;


impl Pconf {
    pub fn to_config(&self, password: &str) -> Result<Config, Error> {
        let token = match &self.encripted {
            Some(false) | 
            None => self.token.to_owned(),
            Some(true) => Cripto::decript(&self.token, password)?,
        };
        
        Ok(
            Config::new(
                self.name.to_owned(),
                self.owner.to_owned(),
                token,
                self.endpoint.to_owned(),
            )
        )
    }
}
