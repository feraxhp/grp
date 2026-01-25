

use futures::future::join_all;

use crate::animation::Animation;
use crate::config::Config;
use crate::platform::Platform;
use crate::error::structs::Error;
use crate::common::pagination::pagination;
use crate::common::structs::{Context, Repo, RequestType};

impl Platform {
    #[allow(dead_code)]
    pub async fn list_repos<T: Into<String>, A: Animation + ?Sized>(&self,
        owner: Option<T>, 
        config: &Config,
        animation: &Box<A>
    ) -> (Vec<Repo>, Option<Error>, Vec<Error>) {
        let header_map = self.get_auth_header(&config.token);
        let owner = owner.map(|o| o.into());
        let owner = owner.unwrap_or(config.user.clone());
        
        animation.change_message("getting user type");
        
        let user_type = match self.get_user_type(&owner, &config).await {
            Ok(ut) => ut,
            Err(e) => return (Vec::new(), Some(e), vec![])
        };
        
        animation.change_message("fetching repositories...");
        
        let url = self.url_list_repos(&user_type, &config.endpoint).await;
        let (responses, error) = pagination(url, header_map).await;
        
        let responses: Vec<_> = responses.into_iter()
            .map(|response| {
                let context = Context {
                    request_type: RequestType::List,
                    owner: Some(user_type.get_user().name),
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
        
        let mut repos_erros: Vec<Error> = response_erros.into_iter().map(Result::unwrap_err).collect();
        let mut repos: Vec<Repo> = Vec::new();
        
        animation.change_message("formating repositories");
        
        for response in responses {
            match self.get_repo(response) {
                Ok(r) => repos.extend(r),
                Err(e) => repos_erros.push(e),
            }
        }
        
        (repos, error, repos_erros)
    }
    
    pub fn get_repo(&self, response: Result<String, Error>) -> Result<Vec<Repo>, Error> {
        match response {
            Ok(rs) => Repo::from_text_array(&rs, &self),
            Err(_) => { unreachable!() }
        }
    }
}