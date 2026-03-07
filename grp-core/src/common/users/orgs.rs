use std::{future, vec};
use futures::StreamExt;

use crate::animation;
use crate::error::structs::Error;
use crate::common::users::structs::User;
use crate::config::Config;
use crate::platform::Platform;

impl Platform {
    
    /// # Return
    /// a vector of `grp_core::structs::User`
    /// 
    /// # Error
    /// a `grp_core::Error` containing the detail of the error. 
    pub async fn get_logged_orgs(&self, config: &Config) -> Result<Vec<User>, Error> {
        let an = Box::new(animation::None);
        
        let mut error: Option<Error> = None;
        let orgs: Vec<User> = self.list_orgs(config, &an)
            .take_while(|result| {
                match result {
                    Ok(_) => future::ready(true),
                    Err(e) => {
                        error = Some(e.clone());
                        future::ready(false)
                    },
                }
            })
            .fold(vec![], async move |acc, act| {
                let mut curr = acc;
                if let Ok(act) = act { curr.extend(act) };
                curr
            })
            .await
        ;
        
        match (orgs.len(), error) {
            (0, Some(e)) => Err(e),
            (_, _) => Ok(orgs)
        }
    }
}
