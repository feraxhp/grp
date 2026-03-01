use color_print::cformat;

use grp_core::Error;

use super::structs::Usettings;
use crate::system::{directories::{Config, Directories}, file::File};

impl Usettings {
    pub(crate) fn save(&self) -> Result<(), Error> {
        let path = Config::file()?;
        
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| Error::new_custom(
                "Error creating the configuration file".to_string(), 
                vec![
                    cformat!("<r>* Error:</> {:?}", e)
                ]
            ))?;
        
        File::write(&path, &contents)
    }
}
