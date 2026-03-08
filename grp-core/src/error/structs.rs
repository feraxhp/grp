use std::fmt::Display;

use color_print::cformat;

use crate::error::tools::Notes;

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
#[derive(Debug, Clone)]
pub struct Error {
    pub(super) etype: String,
    pub message: String,
    pub content: Vec<String>,
}

#[allow(dead_code)]
impl Error {
    pub fn new<P, K, M>(
        etype: &'static str, 
        message: M,
        detail: P,
        explanation: Vec<String>,
        notes: Vec<K>,
    ) -> Error
    where 
        P: Display,
        M: Display,
        K: Display,
    {
        let notes_legth = notes.len();
        let mut content: Vec<String> = Vec::with_capacity(notes_legth + 1 + explanation.len());
        
        content.push(cformat!("<y>{}</>", detail));
        content.extend(explanation);
        
        if notes_legth > 0 { 
            content.push(cformat!(""));
            content.push(cformat!("<g># notes</>"));
            content.extend(notes.iter().as_notes());
        }
        
        Error { 
            etype: format!("custom::{}", etype), 
            message: message.to_string(), 
            content
        }
    }
    
    pub fn collection(errors: Vec<Error>) -> Error {
        let mut content = Vec::new();
        for error in errors {
            content.push(cformat!("\n*<r>{}</>",error.message));
            content.extend(error.content.iter().map(|s| format!("  {}", s)));
        }
        Error { 
            etype: "".to_string(),
            message: "Multiple errors found".to_string(),
            content: content
        }
    }
    
    pub fn get_type(&self) -> String { self.etype.to_owned() }
    
    pub fn show(&self) { self.show_with_offset(0); }
    pub fn show_with_offset(&self, offset: usize) {
        self.content.iter().for_each(|line| {
            eprintln!("{:width$}{}", "", line, width = offset);
        });
    }
}