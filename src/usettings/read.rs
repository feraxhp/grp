use color_print::cprintln;

use grp_core::{Error, ErrorType};
use super::structs::{Pconf, Usettings};
use crate::system::{directories::{Config, Directories}, file::File};


impl Usettings {
    pub fn get_pconf_by_name(&self, name: &str) -> Option<Pconf> {
        if name == "*" { return self.get_default_pconf(); }
        self.pconfs.iter()
            .find(|pconf| pconf.name == name)
            .cloned()
    }
    
    pub fn get_default_pconf(&self) -> Option<Pconf> {
        self.pconfs.iter()
            .find(|pconf| pconf.name == self.default)
            .cloned()
    }
    
    pub fn get_pconf_or_default(&self, name: &str) -> Option<Pconf> {
        self.get_pconf_by_name(name)
            .or_else(|| self.get_default_pconf())
    }
    
    pub fn read() -> Result<Usettings, Error> {
        let mut path = Config::file()?;
        let file = File::read(&path)?;
    
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