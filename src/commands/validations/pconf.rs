use grp_core::animation;
use color_print::cformat;

use crate::usettings::structs::{Pconf, Usettings};
use crate::commands::validations::structure::{Raw, Validations};


impl Validations for Pconf {
    type Output = Pconf;
    
    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        let usettings = Usettings::read().map_err(|e| e.message)?;
        
        if usettings.pconfs.is_empty() { 
            return Err(cformat!("\n* No <m,i>pconfs</> configured, please configure some")) 
        }
        
        let posible_values: String = usettings.pconfs.iter()
            .flat_map( | p |  vec![cformat!("<g>{}</>, ", p.name.clone())])
            .collect();
        
        match usettings.get_pconf_by_name(value, &Box::new(animation::None)) {
            Ok(pconf) => Ok(pconf),
            Err(error) => {
                match error.eq("custom::grp::ussetings::nopconf") {
                     true => Err(cformat!("\n* Posible values are {:?}", posible_values)),
                    false => Err(error.message)
                }
            },
        }
    }
    
    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        let usettings = Usettings::read().map_err(|e| e.message)?;
        
        if usettings.pconfs.is_empty() { 
            return Err(cformat!("\n* No <m,i>pconfs</> configured, please configure some")) 
        }
        
        let posible_values: Vec<String> = usettings.pconfs.iter().map( | p |  cformat!("<g>{}</>", p.name.clone())).collect();
        
        match usettings.get_pconf_by_name(value, &Box::new(animation::None)) {
            Ok(pconf) => Ok(pconf),
            Err(error) => {
                match error.eq("custom::grp::ussetings::nopconf") {
                     true => Err(cformat!("\n* Posible values are {:?}", posible_values)),
                    false => Err(error.message)
                }
            },
        }
    }
}

impl Validations for Raw<Pconf> {
    type Output = Pconf;
    
    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        let usettings = Usettings::read().map_err(|e| e.message)?;
        
        if usettings.pconfs.is_empty() { 
            return Err(cformat!("\n* No <m,i>pconfs</> configured, please configure some")) 
        }
        
        let posible_values: String = usettings.pconfs.iter()
            .flat_map( | p |  vec![cformat!("<g>{}</>, ", p.name.clone())])
            .collect();
        
        match usettings.get_raw_pconf_by_name(value) {
            Some(pconf) => Ok(pconf.to_owned()),
            None => Err(cformat!("\n* Posible values are {:?}", posible_values)),
        }
    }
    
    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        let usettings = Usettings::read().map_err(|e| e.message)?;
        
        if usettings.pconfs.is_empty() { 
            return Err(cformat!("\n* No <m,i>pconfs</> configured, please configure some")) 
        }
        
        let posible_values: Vec<String> = usettings.pconfs.iter().map( | p |  cformat!("<g>{}</>", p.name.clone())).collect();
        
        match usettings.get_raw_pconf_by_name(value) {
            Some(pconf) => Ok(pconf.to_owned()),
            None => Err(cformat!("\n* Posible values are {:?}", posible_values)),
        }
    }
}
