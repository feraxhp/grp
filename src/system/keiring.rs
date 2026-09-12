use keyring::{Entry, Error};

const SERVICE_NAME: &str = "token_vault";
const KEY_USER: &str = "master_user";

#[allow(unused)]
pub struct Keiring;

#[allow(unused)]
impl Keiring {
    pub fn get_password() -> Result<String, Error> {
        let entry = Entry::new(SERVICE_NAME, KEY_USER)?;
        Ok(entry.get_password()?)
    }
    
    pub fn set_password(password: &str) -> Result<(), Error> {
        let entry = Entry::new(SERVICE_NAME, KEY_USER)?;
        entry.set_password(password)?;
        Ok(())
    }
}
