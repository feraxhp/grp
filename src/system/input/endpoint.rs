use grp_core::Error;
use inquire::validator::Validation;
use inquire::{Autocomplete, CustomUserError, Text};

use crate::system::input::Input;
use crate::errors::ToError;


impl Input {
    pub fn endpoint<M, S>(message : M, description: Option<S>, platform: &str) -> Result<String, Error> 
    where 
        M: AsRef<str>,
        S: AsRef<str>,
    {
        let mut promp = Text::new(message.as_ref()).with_validator(input_empty);
        
        if let Some(desc) = description.as_ref().map(|s| s.as_ref()) {
            promp = promp.with_help_message(desc);
        }
        
        let default = match platform {
            "github" => "api.github.com",
            "gitlab" => "gitlab.com",
            "codeberg" => "codeberg.org",
            _ => ""
        };
        
        if default.is_empty() { promp = promp.with_autocomplete(Completer(default)).with_placeholder(default) }
        else { promp = promp.with_placeholder("example.com") }
        
        promp.prompt()
            .map_err(|e| e.to_error(()))
            .map(|d| d.to_string())
    }
}


#[derive(Clone)]
struct  Completer<T>(T);

impl Autocomplete for Completer<&str> {
    fn get_suggestions(&mut self, _: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        Ok(vec![self.0.to_string()])
    }

    fn get_completion(
        &mut self,
        input: &str,
        _: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        if input.is_empty() { return Ok(Some(self.0.to_string())) }
        
        if self.0.starts_with(input) { return Ok(Some(self.0.to_string())) }

        Ok(None)
    }
}

pub fn input_empty(input: &str) -> Result<Validation, CustomUserError> {
    match input {
        "" => Ok(Validation::Invalid("Input can not be empty".into())),
        _ => Ok(Validation::Valid),
    }
}