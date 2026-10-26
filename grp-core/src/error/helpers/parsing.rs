use mynox::Formater;
use mynox::Context;
use mynox::Extend;
use mynox::Error;
use mynox::error;

use crate::ErrorType;


pub trait ParsingUrl<T> {
    fn as_parsing_url_error(self, url: &str) -> Result<T, Error>;
}

impl<T> ParsingUrl<T> for Result<T, url::ParseError> {
    fn as_parsing_url_error(self, url: &str) -> Result<T, Error> {
        self.context(ErrorType::parsing::url, "Error parsing the url")
            .note(format!("url: `'{url}'`"))
    }
}

pub fn parsing_pull_request_as_issue() -> Error {
    error!(
        ErrorType::parsing::issue,
        "Error parsing the issue",
        "The issue you are trying to parse is a **pull_request**",
        => "You have to parse it to a =PullRequest= struct".as_tip()
    )
}
