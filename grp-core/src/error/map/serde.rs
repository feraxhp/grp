use crate::error::{errors::parsing::Parsing, structs::Error};
use serde_json::Error as JsonError;

impl Error {
    pub fn from_serde(text: &String) -> impl Fn(JsonError) -> Error {
        move |e: JsonError| Parsing::serde(e, text)
    }
}