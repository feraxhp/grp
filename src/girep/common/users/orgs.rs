use crate::girep::animation;
use crate::girep::error::structs::Error;
use crate::girep::common::users::structs::User;
use crate::girep::config::Config;
use crate::girep::platform::Platform;

impl Platform {
    pub async fn get_logged_orgs(&self, config: &Config) -> Result<Vec<User>, Error> {
        match self.list_orgs::<animation::None>(config, None).await {
            (orgs, None, _) if !orgs.is_empty() => Ok(orgs),
            (orgs, None, err) if orgs.is_empty() && !err.is_empty() => {
                Err(Error::colection(err))
            },
            (_, Some(e), _) => Err(e),
            (_, None, _) => Ok(Vec::new())
        }
    }
}
