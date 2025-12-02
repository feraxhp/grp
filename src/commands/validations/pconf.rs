use color_print::cformat;

use crate::girep::usettings::structs::{Pconf, Usettings};
use crate::commands::validations::structure::Validations;


impl Validations for Pconf {
    type Output = Pconf;
    
    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        let usettings = Usettings::read().map_err(|e| e.message)?;
        
        if usettings.pconfs.len() == 0 { 
            return Err(cformat!("\n* No <m,i>pconfs</> configured, please configure some")) 
        }
        
        let posible_values: String = usettings.pconfs.iter()
            .flat_map( | p |  vec![cformat!("<g>{}</>, ", p.name.clone())])
            .collect();
        
        match usettings.get_pconf_by_name(value) {
            Some(e) => Ok(e),
            
            None if value.eq(".") && !usettings.default.is_empty() 
            => Ok(usettings.get_default_pconf().unwrap()),
            
            None 
            => Err(cformat!("\n* Posible values are [{posible_values}] and '<m>.</>' for default")),
        }
    }
    
    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        let usettings = Usettings::read().map_err(|e| e.message)?;
        
        if usettings.pconfs.len() == 0 { 
            return Err(cformat!("\n* No <m,i>pconfs</> configured, please configure some")) 
        }
        
        let posible_values: Vec<String> = usettings.pconfs.iter().map( | p |  cformat!("<g>{}</>", p.name.clone())).collect();
        
        match usettings.get_pconf_by_name(value) {
            Some(e) => Ok(e),
            None => Err(cformat!("\n* Posible values are {:?}", posible_values)),
        }
    }
}