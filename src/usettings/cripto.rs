use grp_core::Error;

use crate::system::cripto::Cripto;
use crate::usettings::structs::Usettings;
use crate::animations::animation::Suspend;



impl Usettings {
    pub fn cipher_pconf<A: Suspend + ?Sized>(&mut self, name: &str, animation: &Box<A>) -> Result<bool, Error> {
        let mut modified = false;
        
        let _self = Self {
            default: String::new(),
            pconfs: vec![],
            keyring: self.keyring.clone(),
            hidepass: self.hidepass.clone(),
        };
        
        for pconf in self.pconfs.iter_mut() {
            if pconf.name != name { continue; }
            
            if pconf.encripted { break; }
            let passw = _self.get_password(true, animation)?;
            
            let token = Cripto::encript(&pconf.token, &passw)?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
            break;
        }
        
        Ok(modified)
    } 
    
    pub fn cipher_all_pconfs<A: Suspend + ?Sized>(&mut self, animation: &Box<A>) -> Result<bool, Error> {
        let mut modified = false;
        let mut passw = None;
        
        let _self = Self {
            default: String::new(),
            pconfs: vec![],
            keyring: self.keyring.clone(),
            hidepass: self.hidepass.clone(),
        };
        
        for pconf in self.pconfs.iter_mut() {
            if pconf.encripted { continue; }
            if passw.is_none() { passw = Some(_self.get_password(true, animation)?); };
            
            let token = Cripto::encript(&pconf.token, passw.as_ref().unwrap())?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
        }
        
        Ok(modified)
    } 
}
