use color_print::{cformat, cprintln};

use grp_core::error::structs::Error;
use grp_core::error::types::ErrorType;
use super::structs::{Pconf, Usettings};
use crate::system::directories::Directories;


impl Usettings {
    pub fn get_pconf_by_name(&self, name: &str) -> Option<Pconf> {
        if name == "*" { return self.get_default_pconf(); }
        self.pconfs.iter().find(|pconf| pconf.name == name).cloned()
    }
    pub fn get_default_pconf(&self) -> Option<Pconf> {
        self.pconfs.iter().find(|pconf| pconf.name == self.default).cloned()
    }
    pub fn get_pconf_or_default(&self, name: &str) -> Option<Pconf> {
        self.get_pconf_by_name(name).or_else(|| self.get_default_pconf())
    }
    pub fn read() -> Result<Usettings, Error> {
        let mut path = Directories::config_file()?;
        
        let file = match std::fs::read_to_string(&path) {
            Ok(file) => file,
            Err(e) => return Err(
                Error::new_custom(
                    "The config file could not be read".to_string(),
                    vec![
                        cformat!("* Error : {}", e),
                        cformat!("  Please check the config file at <i,u,b>{:?}</>", path),
                    ]
                )
            )
        };
    
        if file.is_empty() {
            let void_config = Usettings {
                default: "<repo-name>".to_string(),
                pconfs: vec![]
            };
    
            let _ = void_config.save()?;
            
            cprintln!("* The config file has been created at <i,u,b>{}</>", path.as_mut_os_str().to_str().unwrap());
            cprintln!("  To configure it run");
            cprintln!("  <g>• grp config add</>");
    
            return Ok(void_config);
        } 
    
        let config: Usettings = match serde_json::from_str(&file) {
            Ok(json) => json,
            Err(e) => {
                return Err(
                    Error::new(
                        ErrorType::UsettingsParsing,
                        vec![
                            format!("{:?}", e).as_str(),
                            path.as_mut_os_str().to_str().unwrap()
                        ]
                    )
                )
            }
        };
    
        Ok(config)
    }
}