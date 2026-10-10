use grp_core::Error;
use grp_core::animation::Animation;

use crate::animations::animation::Subprogress;
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
    
    pub fn cipher_all_pconfs<A: Animation + Subprogress + ?Sized>(&mut self, password: &str, animation: &mut Box<A>) -> Result<bool, Error> {
        let mut modified = false;
        let index = animation.add();
        animation.set_total(index, self.pconfs.len() as u64, 
            "{pos:.red} of {len:.blue}  {bar:30.green/blue} {elapsed_precise:.yellow}"
        );
        
        for (position, pconf) in self.pconfs.iter_mut().enumerate() {
            if pconf.encripted { continue; }
            let token = Cripto::encript(&pconf.token, password)?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
            
            animation.set_state(index, (position + 1) as u64);
        }
        
        Ok(modified)
    } 
    
    pub fn decipher_pconf(&mut self, name: &str, password: &str) -> Result<bool, Error> {
        let mut modified = false;
        
        for pconf in self.pconfs.iter_mut() {
            if !pconf.encripted { break; }
            if pconf.name != name { continue; }
            
            let token = Cripto::decript(&pconf.token, password)?;
            
            pconf.token = token;
            pconf.encripted = false;
            
            modified = true;
            break;
        }
        
        Ok(modified)
    } 
    
    pub fn decipher_all_pconfs<A: Animation + Subprogress + ?Sized>(&mut self, password: &str, animation: &mut Box<A>) -> Result<bool, Error> {
        let mut modified = false;
        let index = animation.add();
        animation.set_total(index, self.pconfs.len() as u64, 
            "{pos:.red} of {len:.blue}  {bar:30.green/blue} {elapsed_precise:.yellow}"
        );
        
        for (position, pconf) in self.pconfs.iter_mut().enumerate() {
            if !pconf.encripted { continue; }
            
            let token = Cripto::decript(&pconf.token, password)?;
            
            pconf.token = token;
            pconf.encripted = false;
            
            modified = true;
            
            animation.set_state(index, (position + 1) as u64);
        }
        
        Ok(modified)
    } 
    
    pub fn change_password<A: Animation + Subprogress + ?Sized>(&mut self, old: &str, new: &str, animation: &mut Box<A>) -> Result<bool, Error> {
        let mut modified = false;
        let index = animation.add();
        animation.set_total(index, self.pconfs.len() as u64, 
            "{pos:.red} of {len:.blue}  {bar:30.green/blue} {elapsed_precise:.yellow}: {msg}"
        );
        
        for (position, pconf) in self.pconfs.iter_mut().enumerate() {
            if !pconf.encripted { continue; }
            
            animation.set_message(index, "Decripting old secret");
            let token = Cripto::decript(&pconf.token, old)?;
            animation.set_message(index, "Encript new secret");
            let token = Cripto::encript(&token, new)?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
            
            animation.set_state(index, (position + 1) as u64);
        }
        
        Ok(modified)
    } 
}
