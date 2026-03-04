
use std::{env, ffi::OsStr};
use super::super::structure::Completer;


pub struct Remote;


impl<'a> Completer for Remote {
    fn canditates(current: &OsStr) -> Vec<String> {
        let prefix = current.to_string_lossy();
        let path = match env::current_dir() {
            Ok(p) => p,
            Err(_) => return vec![],
        };
        
        let repo = match git2::Repository::discover(path) {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        
        let remotes = match repo.remotes() {
            Ok(sa) => sa,
            Err(_) => return vec![],
        };
        
        if remotes.len() == 0 { return vec![] }
        
        remotes
            .iter()
            .filter_map(|s| {
                let remote = match s {
                    Some(s) => s,
                    None => return None
                };
                
                if prefix.is_empty() || remote.starts_with(&*prefix) {
                    Some(remote.to_string())
                }
                else 
                { None }
            })
            .collect()
    }
}
