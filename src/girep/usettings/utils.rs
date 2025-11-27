use std::process::exit;

use color_print::cformat;

use crate::girep::animation::Animation;
use crate::girep::config::Config;
use super::structs::{Pconf, Usettings};

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

impl Usettings {
    pub fn default_or_exit<A: Animation + ?Sized>(&self, animation: &Box<A>) -> Pconf {
        match self.get_default_pconf() {
            Some(pconf) => pconf,
            None => {
                animation.finish_with_error(cformat!("No default <i,m>pconf</i,m> <r>configured</>"));
                exit(1)
            },
        }
    } 
}