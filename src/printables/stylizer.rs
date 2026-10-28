use grp_core::structs::Author;

use crate::printables::FormatTools;

pub trait Stylizer {
    fn to_link(&self) -> String;
}

impl Stylizer for Author {
    fn to_link(&self) -> String { (&self.name).as_link(&self.url) }
}
