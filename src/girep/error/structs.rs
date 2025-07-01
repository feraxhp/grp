use color_print::cformat;

use crate::girep::error::types::ErrorType;

pub(crate) struct Error {
    pub(crate) message: String,
    pub(crate) content: Vec<String>,
}

#[allow(dead_code)]
impl Error {
    pub(crate) fn new<T: Into<String>>(error: ErrorType, content: Vec<T>) -> Error {
        Error {
            message: error.get_message(),
            content: error.map_content(content.into_iter().map(|s| s.into()).collect()),
        }
    }
    
    pub(crate) fn colection(errors: Vec<Error>) -> Error {
        let mut content = Vec::new();
        for error in errors {
            content.push(cformat!("\n*<r>{}</>",error.message));
            content.extend(error.content.iter().map(|s| format!("  {}", s)));
        }
        Error { 
            message: "Multiple errors found".to_string(),
            content: content
        }
    }

    pub(crate) fn new_custom<T: Into<String>>(message: T, content: Vec<T>) -> Error {
        Error { 
            message: message.into(),
            content: content.into_iter().map(|s| s.into()).collect()
        }
    }

    pub fn show(&self) { self.show_with_offset(0); }
    
    pub fn show_with_offset(&self, offset: usize) {
        self.content.iter().for_each(|line| {
            eprintln!("{:width$}{}", "", line, width = offset);
        });
    }
}