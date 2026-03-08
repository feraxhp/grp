use std::fmt::Display;
use color_print::cformat;

use crate::error::tools::Formater;
use crate::make_error;
use crate::error::structs::Error;

pub struct AlreadyExist;

macro_rules! etype {
    ($literal:literal) => { concat!("already_exist::", $literal) };
}

impl AlreadyExist {
    pub fn repository<P, S, K>(pconf: K, owner: P, repo: S) -> Error 
    where
        P: Display,
        S: Display,
        K: Display,
    {
        let pconf = pconf.concat(":");
        make_error!{
            etype!("repository"), "The repository already exist",
            2 of 
                cformat!("<y>* Please check the file path:</>"), 
                cformat!("  <b><<{}{}/{}>></>", pconf, owner, repo),
        }
    }
    
    #[allow(unused)]
    pub fn user<P, S>(name: P, notes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        make_error!{
            notes, etype!("user"), "The user already exist",
            1 of 
                cformat!("<y>* User: <m><<{}>></>", name),
        }
    }
    
    pub fn organization<P, S>(name: P, notes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        make_error!{
            notes, etype!("organization"), "The organization already exist",
            1 of 
                cformat!("<y>* Org: <m><<{}>></>", name),
        }
    }
    
    #[allow(unused)]
    pub fn custom<P, S>(obj: &'static str, name: P, notes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        make_error!{
            notes, format!("{}::{}", etype!("custom"), obj), format!("The {} already exist", obj),
            1 of 
                cformat!("<y>* {}: <m><<{}>></>", obj, name),
        }
    }
}