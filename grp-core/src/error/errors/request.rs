use std::fmt::Display;

use color_print::cformat;

use crate::make_error;
use crate::error::structs::Error;

macro_rules! etype {
    ($literal:literal) => { concat!("request::", $literal) };
}

pub struct Request;

impl Request {
    pub fn bad_token_scope<P, S>(pconf: P, scopes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        if scopes.is_empty() { panic!("There are no scopes to show") }
        let mut content: Vec<String> = Vec::with_capacity(3 + scopes.len());
        
        content.push(cformat!("<y>* Please check your token.</>"));
        content.push(cformat!("  <g>» Pconf: <m><<{}>></>", pconf));
        content.push(cformat!("  <g>» You must add the following scopes: </>"));
        
        content.extend(scopes.iter().map(|s| { cformat!("    <#e3750e>▸ <m>{}</>", s) }));
        
        Error {
            etype: etype!("bad_token_scope").to_string(),
            message: "The token has insufficient permitions".to_string(),
            content: content,
        }
    }
    
    pub fn fetch<P, S>(message: P, notes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        make_error!{
            notes, etype!("fetch"), message,
            0 of 
        }
    }
    
    pub fn unauthorized<P, S, K>(pconf: P, user: S, notes: Vec<K>) -> Error 
    where
        P: Display,
        S: Display,
        K: Display,
    {
        make_error!{
            notes, etype!("unauthorized"), "The token provided has no access to that resourse",
            2 of 
                cformat!("<y>* Pconf: <m><<{}>></>", pconf),
                cformat!("<y>* User: <m><<{}>></>", user),
        }
    }
    
    pub fn unsuported<P, S, K>(platform: P, action: S, notes: Vec<K>) -> Error 
    where
        P: Display,
        S: Display,
        K: Display,
    {
        make_error!{
            notes, etype!("unsuported"), "The token provided has no access to that resourse",
            1 of 
                cformat!("<y>* <m>{}</m> does not suppot <r,i>{}!</>", platform, action),
        }
    }
    
    pub fn getting_body<P: Display>(error: P) -> Error {
        make_error!{
            etype!("get::body"), "Error getting the response body",
            1 of 
                cformat!("<y>* {}</>", error),
        }
    }
}
