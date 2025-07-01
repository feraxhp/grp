use color_print::cprintln;

use crate::girep::common::show::Show;
use super::structs::Error;

impl Show for Vec<Error> {
    fn print_pretty(&self) {
        if self.is_empty() { return; }
        let size = self.len();
        
        for (i, error) in self.iter().enumerate() {
            cprintln!("<r>* Error ({}/{}): {}</>", i+1, size, error.message);
            
            for content in &error.content {
                cprintln!("  {}", content);
            }
            
            cprintln!();
        }
    }
}