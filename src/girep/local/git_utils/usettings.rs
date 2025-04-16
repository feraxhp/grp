use crate::config::structure::{Pconf, Usettings};

impl Usettings {
    pub(crate) fn get_pconf_from_remote(&self, remote_name: &str) -> Pconf {
        match self.get_pconf(remote_name.to_string()) {
            None => { self.get_default() },
            Some(pconf) => { pconf }
        }
    }
}