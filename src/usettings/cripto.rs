use color_print::cformat;
use grp_core::Error;
use grp_core::Formater;
use grp_core::animation::Animation;
use grp_core::empty_notes;

use crate::animations::animation::Suspend;
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
    
    pub fn change_password<A>(&mut self, old: &str, animation: &mut Box<A>) -> Result<Option<String>, Error> 
    where 
        A: Animation + Subprogress + Suspend + ?Sized,
    {
        let mut modified = false;
        let mut password = None;
        let index = animation.add();
        let length = self.pconfs.len() as u64;
        
        for (position, pconf) in self.pconfs.iter_mut().enumerate() {
            if !pconf.encripted { continue; }
            
            animation.change_message("Decripting old secret");
            
            let token = Cripto::decript(&pconf.token, old)?;
            if password.is_none() { 
                _ = animation.println(cformat!("<y,i>insert</y,i> <m,i>new password</>").as_tip());
                let ns = Usettings { default: String::new(), keyring: false, hidepass: self.hidepass.clone(), pconfs: vec![] };
                let new = ns.get_password(true, &animation)?;
                animation.suspend(||{ eprint!("\x1B[1A\x1B[0J \r"); });
                
                if new == old { 
                    return Err(Error::new(
                        "ussettings::password::change", "The password is identical", 
                        "You can not set the same password!", vec![], empty_notes!()
                    ))
                }
                
                password = Some(new);
                
                animation.set_total(index, length, 
                    "{pos:.red} of {len:.blue}  {bar:30.green/blue} {elapsed_precise:.yellow}"
                );
            }
            animation.change_message("Decripting old secret");
            let token = Cripto::encript(&token, password.as_ref().unwrap())?;
            
            pconf.token = token;
            pconf.encripted = true;
            
            modified = true;
            
            animation.set_state(index, (position + 1) as u64);
        }
        
        match modified {
            true => Ok(password),
            false => Ok(None),
        }
    } 
}
