use std::fmt::Display;

use mynox::{Error, empty_iter};

macro_rules! etype {
    ($literal:literal) => { concat!("grp::general::", $literal) };
}

pub struct GeneralError;


impl GeneralError {
    pub fn invalid_utf8<D: Display>(error: D) -> Error {
        let etype = etype!("invalid_utf8");
        
        Error::new(
            etype, 
            format!("Invalid UTF-8 string parsing"), 
            format!("{error}"),
            empty_iter![], 
            empty_iter!()
        )
    }
}
