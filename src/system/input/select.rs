use std::fmt::Display;

use mynox::Error;
use inquire::Select;

use crate::system::input::Input;
use crate::errors::ToError;


impl Input {
    pub fn select<M, S, D>(message : M, description: Option<S>, options: Vec<D>) -> Result<String, Error> 
    where 
        M: AsRef<str>,
        S: AsRef<str>,
        D: Display,
    {
        let mut promp = Select::new(message.as_ref(), options);
        
        if let Some(desc) = description.as_ref().map(|s| s.as_ref()) {
            promp = promp.with_help_message(desc);
        }
        
        promp.prompt()
            .map_err(|e| e.to_error(()))
            .map(|d| d.to_string())
    }
}
