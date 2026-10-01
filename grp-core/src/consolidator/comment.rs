use crate::Error;
use crate::JSON;
use crate::Platform;
use crate::specific::gitea;
use crate::specific::github;
use crate::specific::gitlab;
use crate::structs::Comment;
use crate::structs::Repo;


impl Comment {
    /// # Return
    /// 
    /// Generates a list of Comments if the information of the text is a valid list of json 
    /// and the platform matches that content.
    /// 
    /// # Error
    /// a `grp_core::Error` of type `grp_core::ErrorType::ResponseParsing`.
    pub fn from_text_array(text: &String, platform: &Platform, repo: Repo) -> Result<Vec<Self>, Error> {
        let comments: Vec<Self> = match platform {
            Platform::Github => {
                let tmp: Vec<github::parser::Comment> = JSON::from_str(text)?;
                
                let comments = tmp.iter().map(|comment| {
                    Comment {
                        id: comment.id.to_owned(),
                        author: comment.user.login.to_owned(),
                        body: comment.body.to_owned(),
                        url: comment.html_url.to_owned(),
                        created_at: comment.created_at.to_owned(),
                        updated_at: comment.updated_at.to_owned(),
                    }
                }).collect();
                
                comments
            },
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => {
                let tmp: Vec<gitea::parser::Comment> = JSON::from_str(text)?;
                
                let comments = tmp.iter().map(|comment| {
                    Comment {
                        id: comment.id.to_owned(),
                        author: comment.user.login.to_owned(),
                        body: comment.body.to_owned(),
                        url: comment.html_url.to_owned(),
                        created_at: comment.created_at.to_owned(),
                        updated_at: comment.updated_at.to_owned(),
                    }
                }).collect();
                
                comments
            },
            Platform::Gitlab => {
                let tmp: Vec<gitlab::parser::Comment> = JSON::from_str(text)?;
                
                let comments = tmp.iter().map(|comment| {
                    Comment {
                        id: comment.id.to_owned(),
                        author: comment.author.name.to_owned(),
                        body: comment.body.to_owned(),
                        url: format!("https://gitlab.com/{}/{}/-/work_items/1#note_{}", repo.name, repo.path, comment.id),
                        created_at: comment.created_at.to_owned(),
                        updated_at: comment.updated_at.to_owned(),
                    }
                }).collect();
                
                comments
            }
        };
        
        Ok(comments)
    }
}
