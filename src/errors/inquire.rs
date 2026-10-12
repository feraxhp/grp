use grp_core::{Error, empty_notes};
use inquire::InquireError;

use crate::errors::ToError;


macro_rules! etype {
    ($literal:literal) => { concat!("inquire::error::", $literal) };
}


impl ToError for InquireError {
    type Context<'a> = ();

    fn to_error<'a>(&self, _: Self::Context<'a>) -> grp_core::Error {
        match self {
            InquireError::NotTTY => {
                Error::new(
                    etype!("NotTTY"), 
                    "The input device is not a TTY", 
                    "imposible to enable raw mode on the terminal", 
                    vec![], empty_notes!()
                )
            },
            InquireError::InvalidConfiguration(e) => Error::new(
                etype!("InvalidConfiguration"), 
                "Prompt configuration is not valid", 
                "Please report this error", 
                vec![], 
                vec![
                    format!("{}", e),
                ]
            ),
            InquireError::IO(error) => Error::new(
                etype!("IO"), 
                "Error while executing IO operations", 
                error, 
                vec![], empty_notes!()
            ),
            InquireError::OperationCanceled => Error::new(
                etype!("OperationCanceled"), 
                "Readding input canceled", 
                "You cancel the operation", vec![], empty_notes!()
            ),
            InquireError::OperationInterrupted => Error::new(
                etype!("OperationInterrupted"), 
                "Readding input canceled", 
                "You kill the operation", vec![], empty_notes!()
            ),
            InquireError::Custom(error) => Error::new(
                etype!("Custom"), 
                "Custom error from Inquire", 
                error, vec![], empty_notes!()
            ),
        }
    }
}