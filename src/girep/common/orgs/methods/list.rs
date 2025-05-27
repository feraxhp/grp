use crate::animations;
use crate::animations::animation::Animation;
use crate::girep::errors::error::Error;
use crate::girep::errors::types::ErrorType;
use crate::girep::common::orgs::org::Org;
use crate::girep::common::repos::paggination::paggination_mannager;
use crate::girep::common::repos::structs::{DebugData, Rtype};
use crate::girep::config::Config;
use crate::girep::github::errors::error_manager;
use crate::girep::platform::Platform;
use futures::future::join_all;

impl Platform {
    pub(crate) async fn list_orgs(&self, config: Config) -> (Vec<Org>, Vec<Error>){
        let header_map = self.get_auth_header(config.token.clone());

        let load_animation = animations::fetch::Fetch::new("Fetching organizations ...");

        let url = self.url_list_orgs(config.endpoint.clone());

        let (responses,mut erros) = paggination_mannager(url, header_map).await;

        let responses: Vec<_> = responses.into_iter().map(|response| {
            error_manager(
                response,
                DebugData{
                    rtype: Rtype::ListOrg,
                    owner: config.pconf.clone(),
                    repo: None,
                },
                config.clone(),
                "Failed to fetch organizations".to_string(),
            )
        }).collect();

        let results = join_all(responses).await;

        let (results, repos_erros): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);

        let repos_erros: Vec<Error> = repos_erros.into_iter().map(Result::unwrap_err).collect();

        erros.extend(repos_erros);

        let mut orgs: Vec<Org> = Vec::new();
        for result in results {
            let result = match result {
                Ok(result) => result,
                Err(e) => {
                    erros.push(e);
                    continue;
                }
            };
            let repository: Vec<Org> = match serde_json::from_str(&result.clone()) {
                Ok(orgs) => orgs,
                Err(e) => {
                    erros.push(Error::new(
                        ErrorType::Dezerialized,
                        vec![
                            e.to_string().as_str(),
                            result.as_str()
                        ]
                    ));
                    continue;
                }
            };
            orgs.extend(repository);
        }

        if erros.is_empty() { load_animation.finish_with_success("Organizations fetched successfully!"); }
        else { load_animation.finish_with_warning("Some Organizations might be missing"); }

        (orgs, erros)
    }
}