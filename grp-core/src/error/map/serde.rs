use crate::error::{structs::Error, types::ErrorType};
use serde_json::Error as JsonError;

impl Error {
    pub fn from_serde(text: &String) -> impl Fn(JsonError) -> Error {
        move |e: JsonError| Error::new(
            ErrorType::ResponseParsing,
            vec![e.to_string(), text.clone()]
        )
    }
}