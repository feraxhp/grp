pub mod not_found;
pub mod request;
pub mod parsing;



use mynox::Error;
use mynox::error;

use crate::ErrorType;
use crate::Platform;


pub fn unsuported(platform: &Platform, action: &str) -> Error {
    error!(
        ErrorType::unsuported::action,
        "This action is unsuported",
        format!("**{}** does not allows you to ={}=", platform, action)
    )
}
