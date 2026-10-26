use grp_core::structs::User;

use crate::printables::FormatTools;

pub trait Stylizer {
    fn to_link(&self) -> String;
}

impl Stylizer for User {
    fn to_link(&self) -> String { (&self.name).as_link(&self.url) }
}
