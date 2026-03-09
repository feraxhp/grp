use grp_core::Error;

use super::structs::Usettings;
use crate::{errors::fs_errors::FSErrors, system::{directories::{Config, Directories}, file::File}};

impl Usettings {
    pub(crate) fn save(&self) -> Result<(), Error> {
        let path = Config::file()?;
        
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| FSErrors::CREATION.file(path.display(), e))?;
        
        File::write(&path, &contents)
    }
}
