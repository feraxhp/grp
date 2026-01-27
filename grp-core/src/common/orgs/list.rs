use futures::future::join_all;

use crate::config::Config;
use crate::platform::Platform;
use crate::animation::Animation;
use crate::error::structs::Error;
use crate::common::users::structs::User;
use crate::common::pagination::pagination;
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
    pub async fn list_orgs<A: Animation + ?Sized>(&self, 
        config: &Config, 
        animation: &Box<A>
    ) -> (Vec<User>, Option<Error>, Vec<Error>) {
        let url = self.url_list_orgs(&config.endpoint);
        let headers = self.get_auth_header(&config.token);
        
        animation.change_message("getting organizations ...");
        let (responses, error) = pagination(url, headers).await;
        
        let responses: Vec<_> = responses.into_iter()
            .map(|response| {
                let context = Context {
                    request_type: RequestType::ListOrg,
                    owner: None,
                    repo: None,
                    additional: None,
                };
                
                self.unwrap(
                    response, "Failed to fetch repositories",
                    &config, context
                )
            }).collect();
        
        let responses = join_all(responses).await;
        
        let (responses, response_erros): (Vec<_>, Vec<_>) = responses.into_iter().partition(Result::is_ok);
        
        let mut orgs_erros: Vec<Error> = response_erros.into_iter().map(Result::unwrap_err).collect();
        let mut orgs: Vec<User> = Vec::new();
        
        animation.change_message("Collecting orgatizacions ...");
        for response in responses {
            match self.get_user(response) {
                Ok(r) => orgs.extend(r),
                Err(e) => orgs_erros.push(e),
            }
        }
        
        (orgs, error, orgs_erros)
    }
    
    pub fn get_user(&self, response: Result<String, Error>) -> Result<Vec<User>, Error> {
        match response {
            Ok(text) => User::from_text_array(&text, self),
            Err(_) => unreachable!(),
        }
    }
}
