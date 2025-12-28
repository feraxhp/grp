use crate::girep::error::types::ErrorType;
use crate::girep::platform::Platform;
use crate::girep::error::structs::Error;
use crate::girep::config::Config;
use crate::girep::common::users::structs::User;
use crate::girep::animation::Animation;
use crate::girep::specific::{gitea, gitlab};


impl Platform {
    pub async fn create_org<T: Into<String>, A: Animation + ?Sized>(&self, 
        name: T,
        config: &Config,
        recursive: bool, 
        animation: &Box<A>
    ) -> (Vec<User>, Vec<Error>) {
        let name = name.into();
        match self {
            Platform::Github => (
                vec![],
                vec![Error::new(
                    ErrorType::Unsupported, 
                    vec![self.name(), "Create Orgs"]
                )]
            ),
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let result = gitea::orgs::create::create(self, &name, config, animation).await;
                match result {
                    Ok(u)  => (vec![u], vec![]),
                    Err(e) => (vec![], vec![e]),
                }
            },
            Platform::Gitlab => gitlab::groups::create::create_group(self, &name, &config, recursive, animation).await
        }
    }
}