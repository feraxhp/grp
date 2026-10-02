use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::common::traits::Convert;
use crate::errors::Parsing;
use crate::specific::gitea;
use crate::specific::github;
use crate::specific::gitlab;
use crate::structs::Issue;


impl Issue {
    /// # Return
    /// Generates an instance of a Issue if the information of 
    /// the text is a valid json and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text<S: AsRef<str>,>(text: &S, platform: &Platform) -> Result<Self, Error> {
        let issue: Self = match platform {
            Platform::Github => {
                let issue: github::parser::Issue = JSON::from_str(text)?;
                match &issue.pull_request {
                    Some(_) => return Err(Parsing::issue_is_pull_request()),
                    None => issue.convert(),
                }
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let issue: gitea::parser::Issue = JSON::from_str(text)?;
                match &issue.pull_request {
                    Some(_) => return Err(Parsing::issue_is_pull_request()),
                    None => issue.convert(),
                }
            },
            Platform::Gitlab => {
                let issue: gitlab::parser::Issue = JSON::from_str(text)?;
                issue.convert()
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
        let issues: Vec<Self> = match platform {
            Platform::Github => {
                let issues: Vec<github::parser::Issue> = JSON::from_str(text)?;
                issues.into_iter().filter_map(|i| {
                    match &i.pull_request {
                        Some(_) => None,
                        None => Some(i.convert()),
                    }
                }).collect()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let issues: Vec<gitea::parser::Issue> = JSON::from_str(text)?;
                issues.into_iter().filter_map(|i| {
                    match &i.pull_request {
                        Some(_) => None,
                        None => Some(i.convert()),
                    }
                }).collect()
            },
            Platform::Gitlab => {
                let issues: Vec<gitlab::parser::Issue> = JSON::from_str(text)?;
                issues.into_iter().map(|i| i.convert()).collect()
            },
        };
        
        Ok(issues)
    }
}
