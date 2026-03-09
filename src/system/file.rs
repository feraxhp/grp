use std::{fs::{read_to_string, write}, path::Path};

use grp_core::Error;

use crate::errors::fs_errors::FSErrors;


pub struct File;

impl File {
    pub fn read(path: &Path) -> Result<String, Error> {
        read_to_string(&path)
            .map_err(|e| FSErrors::READING.file(path.display(), e))
    }
    
    pub fn write(path: &Path, contents: &str) -> Result<(), Error> {
        write(path, contents)
            .map_err(|e| FSErrors::WRITING.file(path.display(), e))
    }
}