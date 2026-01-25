use crate::error::structs::Error;
use crate::error::types::ErrorType;
use crate::common::users::structs::{User, UserType};
use crate::config::Config;
use crate::platform::Platform;
use crate::specific::gitlab;

impl Platform {
    pub async fn get_user_type(&self, name: &String, conf: &Config) -> Result<UserType, Error> {
        let name = name.clone();
        
        let logged_user = self.get_logged_user(conf).await?;
        if name == logged_user.name { return Ok(UserType::LoggedUser(logged_user)); }
        
        match &self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let user = User { id: name.clone(), name: name.clone(), path: None };
                let orgs = self.get_logged_orgs(conf).await?;
                if orgs.iter().any(|org| org.name == name) { 
                    return Ok(UserType::LoggedOrg(user));
                }
                return Ok(UserType::UnloggedUser(user));
            },
            Platform::Gitlab => {
                let orgs = self.get_logged_orgs(conf).await?;
                if orgs.iter().any(|org| org.name == name) { 
                    let user = orgs.iter().find(|org| org.name == name).unwrap();
                    return Ok(UserType::LoggedOrg(user.clone()));
                }
                if orgs.iter().any(|org| org.path.clone().unwrap() == name) {
                    let user = orgs.iter().find(|org| org.path.clone().unwrap() == name).unwrap().to_owned();
                    return Ok(UserType::LoggedOrg(user.clone()));
                }
                match gitlab::users::search::by_name(&self, &name, conf).await? {
                    Some(u) => return Ok(UserType::UnloggedUser(u)),
                    None => (),
                }
                match gitlab::groups::search::by_full_path(&self, &name, conf).await? {
                    Some(u) => Ok(UserType::UnloggedOrg(u)),
                    None => return Err(Error::new(
                        ErrorType::NotOwnerFound,
                        vec![name]
                    )),
                }
            },
        }
    }
}
