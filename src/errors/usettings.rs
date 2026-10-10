use grp_core::Error;
use color_print::cformat;

use crate::system::directories::{Config, Directories};


pub struct UsettingsError;

impl UsettingsError {
    pub fn no_pconf(name: Option<&str>) -> Error {
        let message = match name {
            Some(name) => cformat!("No pconf named <m>{}</>", name),
            None => cformat!("The <i>default</> pconf is not configured"),
        };
        
        let file = match Config::file() {
            Ok(s) => s,
            Err(e) => return e,
        };
        
        Error::new(
        "grp::ussetings::nopconf", 
            message, 
            "verify your configuration file", 
            vec![], 
            vec![
                cformat!("config path: <b, i>{}</>", file.as_os_str().to_string_lossy())
            ]
        )
    }
}
