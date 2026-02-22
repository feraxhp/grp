use grp_core::Error;
use grp_core::structs::User;
use std::collections::{HashMap, HashSet};

use crate::cache::structure::{Cacher, Hasher, Uncacher, Values};

impl Hasher for Vec<User> {
    fn to_set(&self) -> HashSet<String> {
        let mut hash = HashSet::new();
        self.iter()
            .map(|s| {
                match &s.path {
                    Some(p) => p.clone(),
                    None => s.name.clone(),
                }
            })
            .for_each(|s| { hash.insert(s); });
        
        hash
    }

    fn to_values(&self) -> Values {
        Values { 
            users: self.to_set(), 
            repos: HashSet::new()
        }
    }
}

impl Cacher for Vec<User> {
    fn get(pconf: &str) -> Result<HashSet<String>, Error> {
        let values = Self::load()?;
        match values.get(pconf) {
            Some(v) => Ok(v.users.clone()),
            None => Ok(HashSet::new()),
        }
    }
    
    fn put(&self, pconf: &str, union: bool) -> Result<HashMap<String, Values>, Error> {
        let mut values = Self::load()?;
        
        match values.get_mut(pconf) {
            Some(p) if union => { 
                p.users = p.users
                    .union(&self.to_set())
                    .cloned()
                    .collect();
            }
            Some(p) => { 
                p.users = self.to_set(); 
            }
            None => {
                values.insert(pconf.to_string(), self.to_values());
            }
        };
        
        Ok(values)
    }
}

impl Uncacher<Vec<User>> for User {
    fn __rm(pconf: &str, name: &str) -> Result<Option<HashMap<String, Values>>, Error> {
        let mut values = Self::load()?;
        
        let removed = match values.get_mut(pconf) {
            Some(p) => p.users.remove(name),
            _ => false
        };
        
        if removed { Ok(Some(values)) }
        else { Ok(None) }
    }
}
