// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::errors::types::ErrorType;

pub(crate) struct Error {
    pub(crate) message: String,
    pub(crate) content: Vec<String>,
}

impl Error {
    pub(crate) fn new(error: ErrorType, content: Vec<&str>) -> Error {
        Error {
            message: error.get_message(),
            content: error.get_content(content),
        }
    }

    pub(crate) fn new_custom(message: String, content: Vec<String>) -> Error {
        Error { message, content }
    }

    pub(crate) fn show(&self) {
        self.content.iter().for_each(|line| {
            eprintln!("{}", line);
        });
    }
}