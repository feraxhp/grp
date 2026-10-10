use color_print::{cformat, cprintln};

use grp_core::{Error, errors::Parsing};
use grp_core::Formater;
use crate::system::cripto::Cripto;
use super::structs::{Pconf, Usettings};
use crate::animations::animation::Suspend;
use crate::system::{directories::{Config, Directories}, file::File};


impl Usettings {
    pub fn unlock_pconf<A: Suspend + ?Sized>(&self, pconf: Option<&Pconf>, animation: &Box<A>) -> Result<Pconf, Error> {
        if pconf.is_none() { return Err(no_pconf(None)) }

        let pconf = pconf.unwrap();
        if !pconf.encripted { return Ok(pconf.to_owned()); }
        
        let password = self.get_password(animation)?;
        let token = Cripto::decript(&pconf.token, &password)?;
        
        self.try_safe_password(&password);
        Ok(Pconf {
            name: pconf.name.to_owned(),
            owner: pconf.owner.to_owned(),
            token,
            r#type: pconf.r#type.to_owned(),
            endpoint: pconf.endpoint.to_owned(),
            encripted: pconf.encripted.to_owned(),
        })
    }

    pub fn get_pconf_name_by_name(&self, name: &str) -> Option<&Pconf> {
        if name == "*" { return self.get_default_pconf_name() }
        
        self.pconfs.iter().find(|pconf| pconf.name == name)
    }

    pub fn get_default_pconf_name(&self) -> Option<&Pconf> {
        self.pconfs.iter().find(|pconf| pconf.name == self.default)
    }
    
    pub fn get_pconf_by_name<A: Suspend + ?Sized>(&self, name: &str, animation: &Box<A>) -> Result<Pconf, Error> {
        if name == "*" { return self.get_default_pconf(animation) }
        
        let pconf = self.pconfs.iter()
            .find(|pconf| pconf.name == name);
        
        self.unlock_pconf(pconf, animation)
    }
    
    pub fn get_default_pconf<A: Suspend + ?Sized>(&self, animation: &Box<A>) -> Result<Pconf, Error> {
        let pconf = self.pconfs.iter()
            .find(|pconf| pconf.name == self.default);
        
        self.unlock_pconf(pconf, animation)
    }
    
    pub fn get_pconf_or_default<A: Suspend + ?Sized>(&self, name: &str, animation: &Box<A>) -> Result<Pconf, Error> {
        self.get_pconf_by_name(name, animation)
            .or_else(|_| self.get_default_pconf(animation))
    }
    
    pub fn read() -> Result<Usettings, Error> {
        let mut path = Config::file()?;
        let file = File::read(&path)?;
    
        if file.is_empty() {
            let void_config = Usettings {
                default: "<repo-name>".to_string(),
                pconfs: vec![],
                keyring: true,
                hidepass: false
            };
    
            let _ = void_config.save()?;
            
            cprintln!("* The config file has been created at <i,u,b>{}</>", path.as_mut_os_str().to_str().unwrap());
            cprintln!("  To configure it run");
            println!("{}", "grp config add".as_command());
    
            return Ok(void_config);
        } 
    
        let config: Usettings = match serde_json::from_str(&file) {
            Ok(json) => json,
            Err(e) => return Err(Parsing::serde(e, &file))
        };
    
        Ok(config)
    }
}

fn no_pconf(name: Option<&str>) -> Error {
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
