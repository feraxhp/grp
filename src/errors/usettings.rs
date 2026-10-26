use mynox::empty_iter;
use mynox::Error;

use crate::system::directories::{Config, Directories};


pub struct UsettingsError;

impl UsettingsError {
    pub fn no_pconf(name: Option<&str>) -> Error {
        let message = match name {
            Some(name) => format!("No pconf named `{}`", name),
            None => format!("The `default` pconf is not configured"),
        };
        
        let file = match Config::file() {
            Ok(s) => s,
            Err(e) => return e,
        };
        
        Error::new(
        "grp::usetings::nopconf", 
            message, 
            "verify your configuration file", 
            empty_iter![], 
            vec![
                format!("config path: _**{}**_", file.as_os_str().to_string_lossy())
            ]
        )
    }
}
