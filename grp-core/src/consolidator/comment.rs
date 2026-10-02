use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::common::traits::Convert;
use crate::specific::gitea;
use crate::specific::github;
use crate::specific::gitlab;
use crate::structs::Comment;


impl Comment {
    /// # Return
    /// Generates an instance of a Comment if the information of 
    /// the text is a valid json and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text<S: AsRef<str>,>(text: &S, platform: &Platform) -> Result<Self, Error> {
        let comment: Self = match platform {
            Platform::Github => {
                let comment: github::parser::Comment = JSON::from_str(text)?;
                comment.convert()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let comment: gitea::parser::Comment = JSON::from_str(text)?;
                comment.convert()
            },
            Platform::Gitlab => {
                let comment: gitlab::parser::Comment = JSON::from_str(text)?;
                comment.convert()
            },
        };
        
        Ok(comment)
    }
    
    /// # Return
    /// 
    /// Generates a list of Comments if the information of the text is a valid list of json 
    /// and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text_array(text: &String, platform: &Platform, repo: String) -> Result<Vec<Self>, Error> {
        let comments: Vec<Self> = match platform {
            Platform::Github => {
                let comments: Vec<github::parser::Comment> = JSON::from_str(text)?;
                comments.into_iter().map(|c| c.convert()).collect()
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let comments: Vec<gitea::parser::Comment> = JSON::from_str(text)?;
                comments.into_iter().map(|c| c.convert()).collect()
            },
            Platform::Gitlab => {
                let comments: Vec<gitlab::parser::Comment> = JSON::from_str(text)?;
                comments.into_iter().map(|c| {
                    let mut cm = c.convert();
                    cm.url = format!("https://gitlab.com/{}/-/work_items/1#note_{}", repo, &c.id);
                    cm
                }).collect()
            },
        };
        
        Ok(comments)
    }
}
