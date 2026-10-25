use std::fmt::Display;

use color_print::cformat;
use grp_core::{Error, empty_notes};


macro_rules! etype {
    ($literal:literal) => { concat!("fs_error::", $literal) };
}

pub enum FSErrors {
    Creation,
    Reading,
    Writing,
}

impl FSErrors {
    fn etype(&self) -> &'static str {
        match self {
            FSErrors::Creation => etype!("create"),
            FSErrors::Reading => etype!("read"),
            FSErrors::Writing => etype!("write"),
        }
    }
    
    fn name_past(&self) -> &'static str {
        match self {
            FSErrors::Creation => "created",
            FSErrors::Reading => "readed",
            FSErrors::Writing => "writed",
        }
    }
    
    pub fn file<P: Display, S: Display>(&self, path: P, error: S) -> Error {
        Error::new(
            self.etype(),
            format!("The file could not be {}", self.name_past()), 
            error,
            vec![
                cformat!("  <<{}>>", path)
            ], 
            empty_notes!()
        )
    }
    
    pub fn directory<P: Display, S: Display>(&self, path: P, error: S) -> Error {
        Error::new(
            self.etype(),
            format!("The directory could not be {}", self.name_past()), 
            error,
            vec![
                cformat!("  <<{}>>", path)
            ],
            empty_notes!()
        )
    }
}