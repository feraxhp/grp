use crate::animation;
use crate::error::structs::Error;
use crate::common::users::structs::User;
use crate::config::Config;
use crate::platform::Platform;

impl Platform {
    
    /// # Return
    /// a vector of `grp_core::structs::User`
    /// 
    /// # Error
    /// a `grp_core::Error` containing the detail of the error. 
    pub async fn get_logged_orgs(&self, config: &Config) -> Result<Vec<User>, Error> {
        let an = Box::new(animation::None);
        match self.list_orgs(config, &an).await {
            (orgs, None, _) if !orgs.is_empty() => Ok(orgs),
            (orgs, None, err) if orgs.is_empty() && !err.is_empty() => {
                Err(Error::colection(err))
            },
            (_, Some(e), _) => Err(e),
            (_, None, _) => Ok(Vec::new())
        }
    }
}
