use chrono::DateTime;
use chrono::Utc;

use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::specific::gitea;
use crate::specific::github;
use crate::specific::gitlab;

#[derive(Debug)]
pub struct Issue {
    pub author: String,
    pub number: u64,
    pub title: String,
    pub state: String,
    pub url: String,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Issue {
    /// # Return
    /// 
    /// Generates a list of Repos if the information of the text is a valid list of json 
    /// and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text_array(text: &String, platform: &Platform) -> Result<Vec<Self>, Error> {
        let issues = match platform {
            Platform::Github => {
                let tmp: Vec<github::parser::Issue> = JSON::from_str(text)?;
                
                let issues = tmp.iter().filter_map(|issue| {
                    match issue.pull_request {
                        None => Some(Issue { 
                            number: issue.number, 
                            author: issue.user.login.to_string(),
                            title: issue.title.to_owned(),
                            state: issue.state.to_owned(),
                            created_at: issue.created_at.to_owned(),
                            updated_at: issue.updated_at.to_owned(),
                            locked: issue.locked.to_owned(),
                            url: issue.html_url.to_owned(),
                        }),
                        Some(_) => None,
                    }
                }).collect();
                
                issues
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let tmp: Vec<gitea::parser::Issue> = JSON::from_str(text)?;
                let issues = tmp.iter().filter_map(|issue| {
                    match issue.pull_request {
                        None => Some(Issue { 
                            number: issue.number, 
                            author: issue.user.login.to_owned(),
                            title: issue.title.to_owned(),
                            state: issue.state.to_owned(),
                            created_at: issue.created_at.to_owned(),
                            updated_at: issue.updated_at.to_owned(),
                            locked: issue.is_locked.to_owned(),
                            url: issue.html_url.to_owned(),
                        }),
                        Some(_) => None,
                    }
                }).collect();
                
                issues
            },
            Platform::Gitlab => {
                let tmp: Vec<gitlab::parser::Issue> = JSON::from_str(text)?;
                
                let issues = tmp.iter().filter_map(|issue| {
                    match issue.issue_type.as_str() {
                        "issue" => Some(Issue { 
                            number: issue.iid, 
                            author: issue.author.name.to_owned(),
                            title: issue.title.to_owned(),
                            state: issue.state.to_owned(),
                            created_at: issue.created_at.to_owned(),
                            updated_at: issue.updated_at.to_owned(),
                            locked: false,
                            url: issue.web_url.to_owned(),
                        }),
                        _ => None,
                    }
                }).collect();
                
                issues
            }
        };
        
        Ok(issues)
    }
}
