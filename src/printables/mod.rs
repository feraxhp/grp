use std::fmt::Display;

pub mod markdown;
pub mod humanize;
pub mod stylizer;
pub mod show;


pub trait FormatTools {
    fn as_link(self, url: &str) -> String;
}

impl<D: Display> FormatTools for D {
    fn as_link(self, url: &str) -> String {
        format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, self)
    }
}
