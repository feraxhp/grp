use color_print::cprintln;

use crate::girep::common::show::Show;
use super::structs::Error;

impl Show for Vec<Error> {
    fn print_pretty(&self) {
        if self.is_empty() { return; }
        
        for (i, error) in self.iter().enumerate() {
            let len = (i+1).to_string().len();
            cprintln!("<r>{}: {}</>", i+1, error.message);
            error.show_with_offset(len+2);
            cprintln!();
        }
    }
}