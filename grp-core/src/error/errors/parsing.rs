use std::fmt::Display;
use color_print::cformat;

use crate::{error::structs::Error, make_error};


pub struct Parsing;
macro_rules! etype {
    ($literal:literal) => { concat!("parsing::", $literal) };
}

impl Parsing {
    pub fn serde<P: Display>(error: P, text: &str) -> Error {
        make_error!{
            etype!("serde"), "Error parsing a serde object",
            2 of 
                cformat!("<y>* {}</>", error),
                cformat!("  {}", text),
        }
    }
    
    pub fn url<P, S>(error: P, url: S) -> Error 
    where 
        P: Display,
        S: Display,
    {
        make_error!{
            etype!("url"), "Error parsing a url",
            2 of 
                cformat!("<y>* {}</>", error),
                cformat!("  <b><<{}>></>", url),
        }
    }
    
    pub fn issue_is_pull_request() -> Error {
        make_error!{
            etype!("issue_is_pull_request"), "Error parsing the issue",
            2 of 
                cformat!("<y>* The issue you are trying to parse is a pull_request</>"),
                cformat!("<y>  You have to parse it to a PullRequest struct</>"),
        }
    }
}
