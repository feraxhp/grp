use git2::Repository;

use super::super::structs::GitUtils;

impl GitUtils {
    pub fn list_tags(repo: &Repository) -> Vec<String> {
        let mut tags: Vec<String> = vec![];
        
        let _ = repo.tag_foreach(|_, name| {
            match String::from_utf8(name.to_vec()) {
                Ok(s) => tags.push(s),
                Err(_) => (),
            };
            true
        });
        
        tags
    }
}