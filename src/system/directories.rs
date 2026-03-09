use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use grp_core::{Error, empty_notes};

use crate::errors::fs_errors::FSErrors;

macro_rules! create {
    ($method:ident) => {{
        let pd = Self::new()?;
        let directory = pd.$method();
        Self::create_dirs(directory)?;
        
        Ok(directory.into())
    }};
    
    ($file:literal) => {{
        let directory = Self::directory()?;
        let path = directory.join($file);
        Self::create_files(&path)?;
        Ok(path)
    }}
}

/// ## Example 
/// This `directory!(Config config_dir "config.json");`
/// 
/// **Expands to:**
/// 
/// ~~~rust
/// pub struct Config;
/// impl Directories for Config {
///     fn file() -> Result<PathBuf, Error> { create!("config.json") }
///     fn directory() -> Result<PathBuf, Error> { create!(config_dir) }
/// }
/// ~~~
macro_rules! directory {
    ($struct:ident $method:ident $name:literal) => {
        pub struct $struct;
        impl Directories for $struct {
            fn file() -> Result<PathBuf, Error> { create!($name) }
            fn directory() -> Result<PathBuf, Error> { create!($method) }
        }
    };
}

directory!(Config config_dir "config.json");
directory!(Cache cache_dir "cache.json");

pub struct BasicDir;
impl BasicDir {
    pub fn current() -> Result<PathBuf, Error> {
        std::env::current_dir()
            .map_err(|e| FSErrors::READING.directory("CURRENT_DIR", e))
    }
}

pub trait Directories {
    fn file() -> Result<PathBuf, Error>;
    fn directory() -> Result<PathBuf, Error>;
    
    fn new() -> Result<ProjectDirs, Error> {
        let project = ProjectDirs::from("", "", "girep");
        project.ok_or(Error::new(
            "not_found::directories", 
            "Internal error", 
            "The system directories can not be determined", 
            vec![],
            empty_notes!()
        ))
    }
    
    fn create_dirs(location: &Path) -> Result<(), Error> {
        match location.exists() {
            true => Ok(()),
            false => {
                create_dir_all(&location).map_err(|e| {
                    FSErrors::CREATION.directory(location.display(), e)
                })?;
                
                Ok(())
            },
        }
    }
    
    fn create_files(path: &Path) -> Result<(), Error> {
        match path.exists() {
            true => Ok(()),
            false => {
                File::create(&path).map_err(|e| {
                    FSErrors::CREATION.file(path.display(), e)
                })?;
                Ok(())
            },
        }
    }
}