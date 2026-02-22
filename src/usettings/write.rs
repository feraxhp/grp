use color_print::cformat;

use grp_core::Error;

use super::structs::Usettings;
use crate::system::directories::{Config, Directories};

impl Usettings {
    pub(crate) fn save(&self) -> Result<(), Error> {
        let file_location = Config::file()?;
        let file = std::fs::File::create(file_location)
            .map_err(|e| Error::new_custom(
                "Error creating the configuration file".to_string(), 
                vec![
                    cformat!("<r>* Error:</> {:?}", e)
                ]
            ))?;
        
        serde_json::to_writer_pretty(file, self)
            .map_err(|e| Error::new_custom(
                "Error creating the configuration file".to_string(), 
                vec![
                    cformat!("<r>* Error:</> {:?}", e)
                ]
            ))?;
        
        Ok(())
    }
}
