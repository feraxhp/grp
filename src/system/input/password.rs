use grp_core::Error;
use inquire::Password;
use inquire::PasswordDisplayMode;

use crate::errors::ToError;
use crate::system::input::Input;


impl Input {
    pub fn get_password(mode: PasswordDisplayMode, confirm: bool) -> Result<String, Error>{
        let mut prompt = Password::new("Enter password:").with_display_mode(mode);
        
        if !confirm { prompt = prompt.without_confirmation(); }
        
        let password = prompt.prompt().map_err(|e| e.to_error(()));
        eprint!("\x1B[2A\x1B[0J"); // Clear prompt
        password
    }
}
