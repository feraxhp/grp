use color_print::cformat;
use mynox::Error;
use inquire::Password;
use inquire::PasswordDisplayMode;

use crate::errors::ToError;
use crate::system::input::Input;


impl Input {
    pub fn get_password(mode: PasswordDisplayMode, confirm: bool) -> Result<String, Error> {
        let msg = cformat!("<y,i>password</>");
        let confirmation = match confirm {
            true => Some(cformat!("<y,i>password</>")),
            false => None,
        };
        
        let password = Self::password(msg, confirmation, mode);
        eprint!("\x1B[2A\x1B[0J"); // Clear prompt
        password
    }
    
    pub fn password<M, S>(message : M, confirmation: Option<S>, mode: PasswordDisplayMode) -> Result<String, Error> 
    where 
        M: AsRef<str>,
        S: AsRef<str>,
    {
        let mut prompt = Password::new(message.as_ref())
            .with_display_mode(mode);
        
        if let Some(s) = confirmation.as_ref().map(|s| s.as_ref()) { 
            prompt = prompt.with_custom_confirmation_message(s.as_ref()) 
        }
        else { prompt = prompt.without_confirmation() }
        
        prompt.prompt().map_err(|e| e.to_error(()))
    }
}
