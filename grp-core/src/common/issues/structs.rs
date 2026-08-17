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
                    match issue.pullrequest {
                        None => Some(Issue { 
                            number: issue.number, 
                            author: "Placeholder".to_string(),
                            title: issue.title.clone(),
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
                    match issue.pullrequest {
                        None => Some(Issue { 
                            number: issue.number, 
                            author: issue.original_author.to_owned(),
                            title: issue.title.clone(),
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
