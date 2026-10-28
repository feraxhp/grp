use futures::{Stream, StreamExt};
use mynox::Error;

use crate::common::structs::RequestType;
use crate::structs::PaginatorResult;
use crate::common::traits::Convert;
use crate::animation::Animation;
use crate::platform::Platform;
use crate::specific::forgejo;
use crate::specific::gitlab;
use crate::specific::github;
use crate::config::Config;
use crate::structs::User;
use crate::JSON;


impl Platform {
    /// List all the orgs in wich the logged user is member.
    /// 
    /// - `config`: a `grp_core::Config`
    /// - `animation`: a struct wich implements the trait `grp_core::animation::Animation`
    /// 
    /// # Return
    /// a tuple with:
    /// 1. `Vec<Repo>` a list with the repos 
    /// 2. `Option<Error>` a `grp_core::Error` containing the detail of the error, if this error is present, the list of repos is empty.
    /// 3. `Vec<Error>` a list of `grp_core::Error` that contains a list of errors if something happen during the paggination.
    /// 
    /// ## Why is this?
    /// a better solution with yield is planned, but for know this is the best i could do.
    pub fn list_orgs<A: Animation + ?Sized>(&self, 
        config: &Config, 
        animation: &Box<A>
    ) -> impl Stream<Item = Result<PaginatorResult<Vec<User>>, Error>> {
        let url = self.url_list_orgs(config);
        
        animation.change_message("getting organizations ...");
        self.pagginate(url, config, RequestType::ListOrg { way: "by_url".to_string() }, 1)
            .map(|result| {
                self.get_user(result)
            })
    }
    
    pub fn get_user(&self, response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<User>>, Error> {
        let pagginator = response?;
        
        let users = match self {
            Platform::Github => {
                let users: Vec<github::parser::User> = JSON::from_str(&pagginator.result)?;
                users.iter().map(|g|g.convert()).collect()
            },
            Platform::Gitlab => {
                let users: Vec<gitlab::parser::Group> = JSON::from_str(&pagginator.result)?;
                users.iter().map(|g|g.convert()).collect()
            },
            Platform::Gitea |
            Platform::Codeberg |
            Platform::Forgejo => {
                let users: Vec<forgejo::parser::User> = JSON::from_str(&pagginator.result)?;
                users.iter().map(|g|g.convert()).collect()
            },
        };
        
        Ok(pagginator.parse_result(users))
    }
}
