use std::ffi::OsStr;
use grp_core::structs::Repo;

use crate::commands::completions::structure::Completer;
use crate::cache::structure::Cacher;

type VecRepo = Vec<Repo>;

impl Completer for Repo {
    fn canditates(current: &OsStr) -> Vec<String> {
        let current  = match current.to_str() {
            Some(s) => s,
            None => return vec![],
        };
        
        match VecRepo::get("gh") {
            Ok(s) => {
                s.into_iter()
                    .flat_map(|s| {
                        if s.starts_with(current) {
                            Some(s)
                        } 
                        else { None }
                    })
                    .collect()
            },
            Err(_) => vec![],
        }
    }
}