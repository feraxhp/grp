use std::{fs::{read_to_string, write}, path::Path};

use color_print::cformat;
use grp_core::Error;


pub struct File;

impl File {
    pub fn read(path: &Path) -> Result<String, Error> {
        read_to_string(&path)
            .map_err(|e| {
                Error::new_custom("The config file could not be read",
                    vec![
                        cformat!("* Error : {}", e),
                        cformat!("  Please check the config file at <i,u,b>{:?}</>", path),
                    ]
                )
            })
    }
    
    pub fn write(path: &Path, contents: &str) -> Result<(), Error> {
        write(path, contents)
            .map_err(|e| {
                Error::new_custom("The config file could not be read",
                    vec![
                        cformat!("* Error : {}", e),
                        cformat!("  Please check the config file at <i,u,b>{:?}</>", path),
                    ]
                )
            })
    }
}