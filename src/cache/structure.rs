use std::{collections::{HashMap, HashSet}, path::PathBuf};

use color_print::cformat;
use grp_core::{Error, JSON};
use serde::{Deserialize, Serialize};

use crate::system::{directories::{Cache, Directories}, file::File};

#[derive(Debug, Serialize, Deserialize)]
pub struct Values { 
    pub users: HashSet<String>,
    pub repos: HashSet<String>
}

pub trait Cacher {
    fn path() -> Result<PathBuf, Error> { Cache::file() }
    fn load() -> Result<HashMap<String, Values>, Error> {
        let path = Self::path()?;
        let text = File::read(&path)?;
        
        if text.is_empty() { return Ok(HashMap::new()) }
        
        JSON::from_str(&text)
    }
    
    #[allow(unused)]
    fn get(pconf: &str) -> Result<HashSet<String>, Error>;
    fn put(&self, pconf: &str, union: bool) -> Result<HashMap<String, Values>, Error>;
    
    fn save(&self, pconf: &str, union: bool) -> Result<(), Error> {
        let path = Self::path()?;
        let data = self.put(pconf, union)?;
        
        let contents = serde_json::to_string_pretty(&data)
            .map_err(|e| Error::new_custom(
                "Error creating the configuration file".to_string(), 
                vec![
                    cformat!("<r>* Error:</> {:?}", e)
                ]
            ))?;
        
        File::write(&path, &contents)
    }
}

pub trait Uncacher<T>
where 
    T: Cacher
{
    fn load() -> Result<HashMap<String, Values>, Error> { T::load() }
    fn remove(pconf: &str, name: &str) -> Result<bool, Error> { 
        let path = T::path()?;
        let data = match Self::__rm(pconf, name)? {
            Some(d) => d,
            None => return Ok(false),
        };
        
        let contents = serde_json::to_string_pretty(&data)
            .map_err(|e| Error::new_custom(
                "Error creating the configuration file".to_string(), 
                vec![
                    cformat!("<r>* Error:</> {:?}", e)
                ]
            ))?;
        
        File::write(&path, &contents)?;
        Ok(true)
    }
    fn __rm(pconf: &str, name: &str) -> Result<Option<HashMap<String, Values>>, Error>;
}

pub trait Hasher { 
    fn to_set(&self) -> HashSet<String>; 
    fn to_values(&self) -> Values;
}
