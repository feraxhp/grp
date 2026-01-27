use color_print::cformat;

use crate::error::types::ErrorType;

/// # Error
/// This is the _error struct_ used in all the lib 
/// it implements some based functionality to show the error 
/// in a beautifull format.
/// 
/// There are to basic ways to create a new error, 
/// with a preset and additional information, or a custom, for 
/// your own error.
/// 
/// ## Example 1
/// ~~~
/// use grp_core::{ErrorType, Error};
/// 
/// Error::new(
///     ErrorType::ResponseParsing,
///     vec!["description", "{}"]
/// );
/// ~~~
/// 
/// Error contains a vector to emulate various parameter to allow a more 
/// usefull message to the user. In this case, the `ErrorType::ResponseParsing` 
/// needs at least 2 parameters, one for the description of the error, 
/// and other for the text that generates the error.
/// 
/// ## Example 2
/// ~~~
/// use grp_core::{ErrorType, Error};
/// 
/// Error::new_custom(
///     "Custom error message",
///     vec!["something whent wrong after...", "don't be afraid"] // here, every item is trated as a new line (if printed)
/// );
/// ~~~
/// 
/// You also can create custom errors using the _build in_ method _new_custom_ 
/// this is usefull if you whant to canvert between error in your aplication, 
/// a good example is present in _https://github.com/feraxhp/grp_, where the 
/// errors from git2 are parsed to grp_core errors, in order to keep the 
/// same error throughout all the aplication. 
/// 
/// ## Example 3
/// ~~~
/// use grp_core::{ErrorType, Error};
/// 
/// let normal_error = Error::new(
///     ErrorType::ResponseParsing,
///     vec!["description", "{}"]
/// );
/// 
/// let error_custom = Error::new_custom(
///     "Custom error message",
///     vec!["something whent wrong after...", "don't be afraid"] // here, every item is trated as a new line (if printed)
/// );
/// 
/// let collection_error = Error::colection(vec![normal_error, error_custom]);
/// ~~~
/// 
/// There is a 3 type and is a collection, this allows to return multiple errors 
/// in just one `grp_core::Error`. usefull for paggination errors, or concations of multiple 
/// of them.
#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub content: Vec<String>,
}

#[allow(dead_code)]
impl Error {
    pub fn new<T: Into<String>>(error: ErrorType, content: Vec<T>) -> Error {
        Error {
            message: error.get_message(),
            content: error.map_content(content.into_iter().map(|s| s.into()).collect()),
        }
    }
    
    pub fn colection(errors: Vec<Error>) -> Error {
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

    pub fn new_custom<T: Into<String>>(message: T, content: Vec<T>) -> Error {
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