use color_print::cformat;

use crate::girep::error::structs::Error;
use crate::system::directories::Directories;
use crate::usettings::structs::Usettings;


impl Usettings {
    pub(crate) fn save(&self) -> Result<(), Error> {
        let file_location = Directories::config_file()?;
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
