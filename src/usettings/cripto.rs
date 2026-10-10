use grp_core::Error;

use crate::system::cripto::Cripto;
use crate::usettings::structs::Usettings;


impl Usettings {
    pub fn cipher_pconf(&mut self, name: &str, password: &str) -> Result<bool, Error> {
        let mut modified = false;
        
        for pconf in self.pconfs.iter_mut() {
            if pconf.encripted { break; }
            if pconf.name != name { continue; }
            
            let token = Cripto::encript(&pconf.token, password)?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
            break;
        }
        
        Ok(modified)
    } 
    
    pub fn cipher_all_pconfs(&mut self, password: &str) -> Result<bool, Error> {
        let mut modified = false;
        
        for pconf in self.pconfs.iter_mut() {
            if pconf.encripted { continue; }
            
            let token = Cripto::encript(&pconf.token, password)?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
        }
        
        Ok(modified)
    } 
}
