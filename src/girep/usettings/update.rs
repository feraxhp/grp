use super::structs::{Pconf, Usettings};
use crate::girep::error::structs::Error;

impl Usettings {
    pub fn add_pconf(&mut self, pconf: Pconf) -> Result<(), Error> {
        self.pconfs.push(pconf);
        self.save()?;
        Ok(())
    }
}