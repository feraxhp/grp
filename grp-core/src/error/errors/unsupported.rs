use std::fmt::Display;
use color_print::cformat;

use crate::Platform;
use crate::make_error;
use crate::error::structs::Error;

pub struct Unsupported;

macro_rules! etype {
    ($literal:literal) => { concat!("unsuported::", $literal) };
}

impl Unsupported {
    pub fn action<P>(action: P, platform: &Platform) -> Error 
    where
        P: Display,
    {
        make_error!{
            etype!("action"), "This action is unsuported by the platform",
            2 of 
                cformat!("<y> * The accion <m>{}</m> is not supported yet", action),
                cformat!("  Platform: <m>{}</m>", platform)
        }
    }
}
