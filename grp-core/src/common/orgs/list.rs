use crate::config::Config;
use crate::platform::Platform;
use crate::animation::Animation;
use crate::error::structs::Error;
use futures::{Stream, StreamExt};
use crate::common::users::structs::User;
use crate::common::structs::{Context, RequestType};


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
    ) -> impl Stream<Item = Result<Vec<User>, Error>> {
        let url = self.url_list_orgs(&config.endpoint);
        
        let context = Context {
            request_type: RequestType::ListOrg,
            owner: None,
            repo: None,
            additional: None,
        };

        animation.change_message("getting organizations ...");
        self.pagginate(url, config, context)
            .map(|result| {
                self.get_user(result)
            })
    }
    
    pub fn get_user(&self, response: Result<String, Error>) -> Result<Vec<User>, Error> {
        match response {
            Ok(text) => User::from_text_array(&text, self),
            Err(e) => Err(e),
        }
    }
}
