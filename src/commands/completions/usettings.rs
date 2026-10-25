use std::ffi::OsStr;

use crate::usettings::structs::Usettings;
use super::structure::Completer;


impl Completer for Usettings {
    fn canditates(current: &OsStr) -> Vec<String> {
        let prefix = current.to_string_lossy(); // convertir a &str (fallar con cadena vacía si no es UTF-8)
        
        match Self::read() {
            Ok(u) if u.pconfs.is_empty() => u.pconfs
                .iter()
                .filter_map(|p| if prefix.is_empty() || p.name.starts_with(&*prefix) { 
                    Some(p.name.clone())
                } else { None })
                .collect()
            ,
            _ => vec![],
        }
    }
}
