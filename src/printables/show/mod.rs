pub mod comment;
pub mod pconf;
pub mod issue;
pub mod repo;
pub mod user;

use color_print::cformat;


pub trait Show {
    type Context;
    
    fn print_pretty(&self, context: Self::Context) {
        self.to_string_iter(context).for_each(|s| {
            println!("{}", s)
        });
    }
    fn to_string_iter(&self, context: Self::Context) -> impl Iterator<Item = String> + '_;
}

impl Show for Vec<String> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
        self.iter().map(|s| s.to_owned())
    }
}

pub trait Empty {
    fn empty(&self) -> String;
}

impl Empty for String {
    fn empty(&self) -> String {
        match self.len() {
            0 => cformat!("<dim,i><<empty>></dim,i>"),
            _ => self.to_string(),
        }
    }
}

impl Empty for &String {
    fn empty(&self) -> String {
        match self.len() {
            0 => cformat!("<dim,i><<empty>></dim,i>"),
            _ => self.to_string(),
        }
    }
}
