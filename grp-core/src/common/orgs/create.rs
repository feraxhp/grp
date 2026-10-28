use mynox::Error;
use mynox::Formater;

use crate::specific::{forgejo, gitlab};
use crate::structs::RequestType;
use crate::animation::Animation;
use crate::platform::Platform;
use crate::config::Config;
use crate::structs::User;
use crate::helpers;


impl Platform {
    /// Create a given **org** for the logged user in the selected platform.
    /// 
    /// - `name`: the name or path of the **org** to be created.
    /// - `config`: a `grp_core::Config`.
    /// - `recursive`: only valid for "Gitlab", it will create all the groups if they not exist. 
    /// - `animation`: a struct wich implements the trait `grp_core::animation::Animation`.
    /// 
    /// # Returns
    /// a tuple with:
    /// 1. `Vec<User>`: A list of the created orgs.
    /// 2. `Vec<Error>`: A list of errors if some org faild to be created.
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
                vec![ 
                    helpers::unsuported(self, RequestType::CreateOrg{ name }.as_ref())
                        .extend_notes([
                            "you have to create the _org manualy on github_".as_tip()
                        ]),
                ]
            ),
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let result = forgejo::orgs::create::create(self, &name, config, animation).await;
                match result {
                    Ok(u)  => (vec![u], vec![]),
                    Err(e) => (vec![], vec![e]),
                }
            },
            Platform::Gitlab => gitlab::groups::create::create_group(self, &name, config, recursive, animation).await
        }
    }
}
