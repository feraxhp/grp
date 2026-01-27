use grp_core::Error;

use super::structs::{Pconf, Usettings};

impl Usettings {
    pub fn add_pconf(&mut self, pconf: Pconf) -> Result<(), Error> {
        self.pconfs.push(pconf);
        self.save()?;
        Ok(())
    }
}