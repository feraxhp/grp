use color_print::ceprint;
use grp_core::{Config, Error};
use inquire::PasswordDisplayMode;
use zeroize::Zeroizing;

use super::structs::Pconf;
use crate::system::input::Input;
use crate::system::keyring::Keyring;
use crate::usettings::structs::Usettings;
use crate::animations::animation::Suspend;


impl Pconf {
    pub fn to_config(&self) -> Config {
        Config::new(
            self.name.to_owned(),
            self.owner.to_owned(),
            (*self.token).clone(),
            self.endpoint.to_owned(),
        )
    }
}

impl Usettings {
    pub fn get_password<A>(&self, confirm: bool, animation: &Box<A>) -> Result<Zeroizing<String>, Error>
    where 
        A: Suspend + ?Sized
    {
        let password: Result<Zeroizing<String>, String> = match self.keyring {
            false => Err( "Skiping keyring...".to_string() ),
            true => {
                match Keyring::get_password() {
                    Ok(p) => Ok(p),
                    Err(e) => Err( e.to_string() ),
                }
            },
        };

        if let Ok(pass) = password { return Ok(pass); };
        
        let mode = match self.hidepass {
            true => PasswordDisplayMode::Hidden,
            false => PasswordDisplayMode::Masked,
        };

        let mut password_: Result<Zeroizing<String>, Error> = Ok(Zeroizing::new(String::new()));
        animation.suspend(|| { 
            ceprint!("<y>{}</>\n", &password.unwrap_err());
            password_ = Input::get_password(mode, confirm)
        });
        
        password_
    }

    pub fn try_save_password(&self, password: &str) {
        match self.keyring {
            false => (),
            true => _ = Keyring::set_password(&password),
        };
    }
}
