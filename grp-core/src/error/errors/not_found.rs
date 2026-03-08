use std::fmt::Display;
use color_print::cformat;

use crate::error::tools::Formater;
use crate::error::structs::Error;
use crate::make_error;

macro_rules! etype {
    ($literal:literal) => { concat!("not_found::", $literal) };
}


pub struct NotFound;

impl NotFound {
    pub fn repository<P, S, K>(pconf: K, owner: P, repo: S) -> Error 
    where
        P: Display,
        S: Display,
        K: Display,
    {
        make_error!{
            etype!("repository"), "The repository does not exist",
            2 of 
                cformat!("<y>* Please check the file path:</>"), 
                cformat!("  <b><<{}{}/{}>></>", pconf.concat(":"), owner, repo),
        }
    }
    
    // pub fn file<P, S>(path: P, notes: Vec<S>) -> Error 
    // where
    //     P: Display,
    //     S: Display,
    // {
    //     make_error!{
    //         notes, etype!("file"), "The file can not be found",
    //         2 of 
    //             cformat!("<y>* Please check the file path:</>"), 
    //             cformat!("  <b><<{}>></>", path),
    //     }
    // }
    
    // pub fn directory<P, S>(path: P, notes: Vec<S>) -> Error 
    // where
    //     P: Display,
    //     S: Display,
    // {
    //     make_error!{
    //         notes, etype!("directory"), "The directory can not be found",
    //         2 of 
    //             cformat!("<y>* Please check the directory:</>"),
    //             cformat!("  <b><<{}>></>", path),
    //     }
    // }
    
    pub fn user<P, S>(name: P, notes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        make_error!{
            notes, etype!("user"), "The user does not exist",
            1 of 
                cformat!("<y>* User: <m><<{}>></>", name),
        }
    }
    
    // pub fn local_repository<P>(path: P) -> Error 
    // where
    //     P: Display,
    // {
    //     make_error!{
    //         etype!("repository::local"), "The repository can not be found",
    //         5 of 
    //             cformat!("<y>* Is this directory a git repository?</>"),
    //             cformat!("  <b><<{}>></>", path),
    //             cformat!(""),
    //             "To create a new repo you can run".as_tip(),
    //             "git init".as_command(),
    //     }
    // }
    
    pub fn organization<P, S>(name: P, notes: Vec<S>) -> Error 
    where
        P: Display,
        S: Display,
    {
        make_error!{
            notes, etype!("organization"), "The organization does not exist",
            1 of 
                cformat!("<y>* Org: <m><<{}>></>", name),
        }
    }
}