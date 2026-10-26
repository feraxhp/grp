use mynox::Error;
use mynox::empty_iter;

use inquire::InquireError;

use crate::errors::ToError;


macro_rules! etype {
    ($literal:literal) => { concat!("inquire::error::", $literal) };
}


impl ToError for InquireError {
    type Context<'a> = ();

    fn to_error<'a>(self, _: Self::Context<'a>) -> mynox::Error {
        let (etype, message, details, notes) = match &self {
            InquireError::NotTTY => (
                etype!("NotTTY"),
                "The input device is not a TTY",
                "imposible to enable raw mode on the terminal".to_string(),
                Vec::new(),
            ),
            InquireError::InvalidConfiguration(e) => (
                etype!("InvalidConfiguration"),
                "Prompt configuration is not valid",
                "Please report this error".to_string(),
                vec![e.clone()],
            ),
            InquireError::IO(error) => (
                etype!("IO"),
                "Error while executing IO operations",
                error.to_string(),
                Vec::new(),
            ),
            InquireError::OperationCanceled => (
                etype!("OperationCanceled"),
                "Readding input canceled",
                "You cancel the operation".to_string(),
                Vec::new(),
            ),
            InquireError::OperationInterrupted => (
                etype!("OperationInterrupted"),
                "Readding input canceled",
                "You kill the operation".to_string(),
                Vec::new(),
            ),
            InquireError::Custom(error) => (
                etype!("Custom"),
                "Custom error from Inquire",
                error.to_string(),
                Vec::new(),
            ),
        };
        
        Error::from(
            self,
            etype,
            message,
            details,
            empty_iter![],
            notes,
        )
    }
}
