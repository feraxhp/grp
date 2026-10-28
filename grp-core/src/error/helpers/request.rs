use std::fmt::Display;

use mynox::Formater;
use mynox::Context;
use mynox::Error;
use mynox::error;

use crate::structs::RequestType;
use crate::ErrorType;


pub(crate) trait RequestError<T> {
    fn getting_body(self) -> Result<T, Error>;
    fn fetch(self, detail: impl Into<String>) -> Result<T, Error>;
}

impl<T> RequestError<T> for Result<T, reqwest::Error> {
    fn getting_body(self) -> Result<T, Error> {
        self.context(ErrorType::request::getting_body, "Error getting the response body")
    }
    
    fn fetch(self, detail: impl Into<String>) -> Result<T, Error> {
        self.context(ErrorType::request::fetch, detail)
    }
}

pub fn unauthorized(request_type: &RequestType) -> Error {
    error!(
        ErrorType::request::unauthorized,
        "401 - Unauthorized",
        "The token provided does not have access to that resourse"
        => request_type.as_tip()
    )
}

pub fn bad_token_scope<I>(scopes: I, request_type: &RequestType) -> Error 
where 
    I: IntoIterator,
    I::Item: Display,
{
    let notes = scopes.into_iter().map(|s| format!("*▸ `{s}`*").as_tip_cotinuation());
    
    error!(
        ErrorType::request::unauthorized,
        "403 - Bad token scope",
        "The token has -insufficient _permitions_-",
        => "Required scopes: ".as_tip()
    )
    .extend_notes(notes)
    .extend_note(request_type.as_tip())
}

pub fn unknown(status: u16, base_message: impl Display, text: Option<String>, request_type: &RequestType) -> Error {
    let mut error = Error::new(
        ErrorType::request::unknown, 
        base_message, 
        "Something went wrong",
        [format!("» **code**: _{}_", status)],
        [
            request_type.as_tip(),
        ],
    );
    
    if let Some(t) = text {
        error = error.extend_detail(format!("» **text**: _{}_", t));
    }
    
    error
}
