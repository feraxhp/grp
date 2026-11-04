use mynox::Error;

use crate::common::traits::Convert;
use crate::structs::PullRequest;
use crate::specific::forgejo;
use crate::specific::github;
use crate::specific::gitlab;
use crate::Platform;
use crate::JSON;


impl PullRequest {
    /// # Return
    /// Generates an instance of a Issue if the information of 
    /// the text is a valid json and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text<S: AsRef<str>,>(text: &S, platform: &Platform) -> Result<Self, Error> {
        let issue: Self = match platform {
            Platform::Github => {
                let pr: github::parser::Pr = JSON::from_str(text)?;
                pr.convert()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { todo!()
                // let issue: forgejo::parser::Issue = JSON::from_str(text)?;
                // match &issue.pull_request {
                //     Some(_) => return Err(parsing_pull_request_as_issue()),
                //     None => issue.convert(),
                // }
            },
            Platform::Gitlab => { todo!()
                // let issue: gitlab::parser::Issue = JSON::from_str(text)?;
                // issue.convert()
            },
        };
        
        Ok(issue)
    }
    
    /// # Return
    /// 
    /// Generates a list of User if the information of the text is a valid list of json 
    /// and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text_array<S: AsRef<str>>(text: &S, platform: &Platform) -> Result<Vec<Self>, Error> {
        let prs: Vec<Self> = match platform {
            Platform::Github => {
                let issues: Vec<github::parser::Pr> = JSON::from_str(text)?;
                issues.into_iter().map(|i| { i.convert() }).collect()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => { todo!()
                // let issues: Vec<forgejo::parser::Issue> = JSON::from_str(text)?;
                // issues.into_iter().filter_map(|i| {
                //     match &i.pull_request {
                //         Some(_) => None,
                //         None => Some(i.convert()),
                //     }
                // }).collect()
            },
            Platform::Gitlab => { todo!()
                // let issues: Vec<gitlab::parser::Issue> = JSON::from_str(text)?;
                // issues.into_iter().map(|i| i.convert()).collect()
            },
        };
        
        Ok(prs)
    }
}
