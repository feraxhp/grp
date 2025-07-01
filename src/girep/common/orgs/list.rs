use futures::future::join_all;

use crate::girep::config::Config;
use crate::girep::platform::Platform;
use crate::girep::animation::Animation;
use crate::girep::error::structs::Error;
use crate::girep::common::users::structs::User;
use crate::girep::common::pagination::pagination;
use crate::girep::common::structs::{Context, RequestType};


impl Platform {
    pub async fn list_orgs<A: Animation + ?Sized>(&self, 
        config: &Config, 
        animation: Option<&Box<A>>
    ) -> (Vec<User>, Option<Error>, Vec<Error>) {
        let url = self.url_list_orgs(&config.endpoint);
        let headers = self.get_auth_header(&config.token);
        
        if let Some(an) = animation { an.change_message("getting organizations ..."); }
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
        
        if let Some(an) = animation { an.change_message("Collecting orgatizacions ..."); }
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
