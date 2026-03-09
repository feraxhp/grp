use std::fmt::Display;

use color_print::cformat;

pub trait Notes: Iterator + Sized {
    fn as_notes(self) -> impl Iterator<Item = String>
    where
        Self::Item: Display
    { 
        self.map(|item| cformat!("  <c>→ <m>{}</>", item))
    }
}

impl<I> Notes for I
where
    I: Iterator,
    I::Item: Display,
{}

#[allow(unused)]
pub trait Formater {
    fn as_command(self) -> String;
    fn as_tip(self) -> String;
    fn as_tip_cotinuation(self) -> String;
    fn concat(self, postfix: &'static str) -> String;
}

impl<D> Formater for D
where 
    D: Display
{
    fn as_command(self) -> String {
        cformat!("  <c>•</><g> {} </>", self)
    }

    fn as_tip(self) -> String {
        cformat!("<g>✔</> {}", self)
    }
    
    fn as_tip_cotinuation(self) -> String {
        format!(" {}", self)
    }

    fn concat(self, postfix: &'static str) -> String {
        format!("{}{}", self, postfix)
    }

    
}