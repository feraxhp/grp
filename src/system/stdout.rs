use std::io::{self, Write};

pub trait Printable {
    fn print(&self);
}

#[allow(dead_code)]
pub fn writeln<T: Printable>(input: T) { 
    input.print();
    println!();
}

#[allow(dead_code)]
pub fn write<T: Printable>(input: T) { 
    input.print();
}

impl Printable for &str {
    fn print(&self) { 
        let _ = io::stdout().write(self.as_bytes());
        let _ = io::stdout().flush();
    }
}

impl Printable for String {
    fn print(&self) {
        let _ = io::stdout().write(self.as_bytes());
        let _ = io::stdout().flush();
    }
}

impl Printable for Vec<&str> {
    fn print(&self) {
        for s in self {
            let _ = io::stdout().write(s.as_bytes());
            let _ = io::stdout().write("\n".as_bytes());
        }
        let _ = io::stdout().flush();
    }
}

impl Printable for Vec<String> {
    fn print(&self) {
        for s in self {
            let _ = io::stdout().write(s.as_bytes());
            let _ = io::stdout().write("\n".as_bytes());
        }
        let _ = io::stdout().flush();
    }
}
