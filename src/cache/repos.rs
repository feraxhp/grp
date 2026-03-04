use grp_core::Error;
use grp_core::structs::Repo;
use std::collections::{HashMap, HashSet};

use crate::cache::structure::{Cacher, Hasher, Uncacher, Values};

impl Hasher for Vec<Repo> {
    fn to_set(&self) -> HashSet<String> {
        let mut hash = HashSet::new();
        self.iter()
            .map(|s| { s.path.clone() })
            .for_each(|s| { hash.insert(s); });
        
        hash
    }

    fn to_values(&self) -> Values {
        Values { 
            repos: self.to_set(), 
            users: HashSet::new()
        }
    }
}

impl Cacher for Vec<Repo> {
    fn get(pconf: &str) -> Result<HashSet<String>, Error> {
        let values = Self::load()?;
        match values.get(pconf) {
            Some(v) => Ok(v.repos.clone()),
            None => Ok(HashSet::new()),
        }
    }

    fn put(&self, pconf: &str, union: bool) -> Result<HashMap<String, Values>, Error> {
        let mut values = Self::load()?;
        
        match values.get_mut(pconf) {
            Some(p) if union => { 
                p.repos = p.repos
                    .union(&self.to_set())
                    .cloned()
                    .collect();
            }
            Some(p) => { 
                p.repos = self.to_set(); 
            }
            None => {
                values.insert(pconf.to_string(), self.to_values());
            }
        };
        
        Ok(values)
    }
}

impl Uncacher<Vec<Repo>> for Repo {
    fn __rm(pconf: &str, name: &str) -> Result<Option<HashMap<String, Values>>, Error> {
        let mut values = Self::load()?;
        
        let removed = match values.get_mut(pconf) {
            Some(p) => p.repos.remove(name),
            None => false
        };
        
        if removed { Ok(Some(values)) }
        else { Ok(None) }
    }
}
