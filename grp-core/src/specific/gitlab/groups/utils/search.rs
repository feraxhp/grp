use crate::specific::gitlab::groups::utils::traits::Search;
use crate::common::users::structs::User;


impl Search for Vec<User> {
    fn search(&self, path: &str) -> Option<User> {
        self.iter().find(|user| user.path == Some(path.to_string())).cloned()
    }
}

pub fn get_closest_parent_and_stack<T: Into<String>>(name: T, vec: &Vec<User>, recursive: bool) -> (Option<User>, Vec<String>) {
    let mut stack = vec![];
    let mut path = name.into();
    
    if let Some(u) = vec.search(&path) { return (Some(u), stack); }
    
    while path.contains("/") {
        match path.rsplit_once("/") {
            Some((p, n)) => {
                stack.push(n.to_string());
                if let Some(u) = vec.search(p) { return (Some(u), stack); }
                path = p.to_string();
            },
            None => break,
        };
        
        if !recursive { break; }
    }
    
    if stack.is_empty() { (None, vec![path]) }
    else { (None, stack) }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Vec<User> {
        vec![
            User{ id: "1".to_string(), path: Some("test1".to_string()),              name: "".to_string()},
            User{ id: "2".to_string(), path: Some("test1/prueba".to_string()),       name: "".to_string()},
            User{ id: "3".to_string(), path: Some("test1/prueba2".to_string()),      name: "".to_string()},
            User{ id: "4".to_string(), path: Some("test2/prueba3/sub1".to_string()), name: "".to_string()},
        ]
    }

    #[test]
    fn test_exact_match() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("test1", &orgs, true);
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().id, "1");
        assert!(stack.is_empty());
    }

    #[test]
    fn test_no_match_single_path() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("nonexistent", &orgs, true);
        assert!(parent.is_none());
        assert_eq!(stack, vec!["nonexistent"]);
    }

    #[test]
    fn test_parent_match_with_stack() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("test1/prueba/subpath", &orgs, true);
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().id, "2");
        assert_eq!(stack, vec!["subpath"]);
    }

    #[test]
    fn test_deep_path_with_existing_parent() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("test2/prueba3/sub1/sub2/sub3", &orgs, true);
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().id, "4");
        assert_eq!(stack, vec!["sub3", "sub2"]);
    }

    #[test]
    fn test_non_recursive_search() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("test1/prueba/subpath", &orgs, false);
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().id, "2");
        assert_eq!(stack, vec!["subpath"]);
    }

    #[test]
    fn test_search_trait() {
        let orgs = setup();
        let result = orgs.search("test1");
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "1");

        let result = orgs.search("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_path_with_multiple_segments() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("test1/prueba2/sub1/sub2/sub3", &orgs, true);
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().id, "3");
        assert_eq!(stack, vec!["sub3", "sub2", "sub1"]);
    }

    #[test]
    fn test_empty_path() {
        let orgs = setup();
        let (parent, stack) = get_closest_parent_and_stack("", &orgs, true);
        assert!(parent.is_none());
        assert_eq!(stack, vec![""]);
    }
}
