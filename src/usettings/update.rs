use grp_core::error::structs::Error;

use super::structs::{Pconf, Usettings};

impl Usettings {
    pub fn add_pconf(&mut self, pconf: Pconf) -> Result<(), Error> {
        self.pconfs.push(pconf);
        self.save()?;
        Ok(())
    }
}