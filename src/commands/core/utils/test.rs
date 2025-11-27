#[cfg(test)]
mod test {
    use crate::girep::usettings::structs::{Pconf, Usettings};
    use crate::commands::core::utils::repo_struct::unfold_repo_structure;
    
    fn get_test_usettings() -> Usettings {
        let github = Pconf {
            name: "gh".to_string(),
            owner: "feraxhp".to_string(),
            token: "".to_string(),
            r#type: "github".to_string(),
            endpoint: "".to_string(),
        };
        
        let gitea = Pconf {
            name: "gt".to_string(),
            owner: "gtuser".to_string(),
            token: "".to_string(),
            r#type: "gitea".to_string(),
            endpoint: "".to_string(),
        };
        
        let gitlab = Pconf {
            name: "gl".to_string(),
            owner: "gluser/path".to_string(),
            token: "".to_string(),
            r#type: "gitlab".to_string(),
            endpoint: "".to_string(),
        };
        
        Usettings {
            pconfs: vec![github, gitea, gitlab],
            default: "gh".to_string(),
        }
    }
    
    fn unfold(value: &str) -> Result<(Option<String>, String, String), String> {
        let usettings = get_test_usettings();
        unfold_repo_structure(value, true, &usettings)
    } 
    
    #[test]
    fn gh_test1() {
        match unfold("gh:feraxhp/test") {
            Ok((Some(pconf), owner, repo_name)) => {
                assert_eq!(pconf, "gh");
                assert_eq!(owner, "feraxhp");
                assert_eq!(repo_name, "test");
            },
            Ok(res)  => {
                panic!("Expected (Some(pconf), owner, repo_name), got {:?}", res);
            }
            Err(e) => panic!("Err: {}", e),
        }
    }

    #[test]
    fn gh_test2() {
        match unfold("gh:feraxhp/prueba/test") {
            Ok(res)  => {
                panic!("Expected to fail, got {:?}", res);
            }
            Err(_) => (),
        }
    }

    #[test]
    fn default_pconf_test() {
        match unfold("*:feraxhp/test") {
            Ok((Some(pconf), owner, repo_name)) => {
                assert_eq!(pconf, "*");
                assert_eq!(owner, "feraxhp");
                assert_eq!(repo_name, "test");
            },
            Ok(res)  => {
                panic!("Expected (Some(pconf), owner, repo_name), got {:?}", res);
            }
            Err(e) => panic!("Err: {}", e),
        }
    }

    #[test]
    fn gh_default_owner_test() {
        match unfold("gh:*/test") {
            Ok((Some(pconf), owner, repo_name)) => {
                assert_eq!(pconf, "gh");
                assert_eq!(owner, "feraxhp");
                assert_eq!(repo_name, "test");
            },
            Ok(res)  => {
                panic!("Expected (Some(pconf), owner, repo_name), got {:?}", res);
            }
            Err(e) => panic!("Err: {}", e),
        }
    }

    #[test]
    fn gl_default_owner_test() {
        match unfold("gl:*/test") {
            Ok((Some(pconf), owner, repo_name)) => {
                assert_eq!(pconf, "gl");
                assert_eq!(owner, "gluser/path");
                assert_eq!(repo_name, "test");
            },
            Ok(res)  => {
                panic!("Expected (Some(pconf), owner, repo_name), got {:?}", res);
            }
            Err(e) => panic!("Err: {}", e),
        }
    }

    #[test]
    fn gl_subgroup_test() {
        match unfold("gl:feraxhp/prueba/test") {
            Ok((Some(pconf), owner, repo_name)) => {
                assert_eq!(pconf, "gl");
                assert_eq!(owner, "feraxhp/prueba");
                assert_eq!(repo_name, "test");
            },
            Ok(res)  => {
                panic!("Expected (Some(pconf), owner, repo_name), got {:?}", res);
            }
            Err(e) => panic!("Err: {}", e),
        }
    }

    #[test]
    fn gt_subgroup_test() {
        match unfold("gt:feraxhp/prueba/test") {
            Ok(res)  => {
                panic!("Expected to fail, got {:?}", res);
            }
            Err(_) => (),
        }
    }
    
    #[test]
    fn gl_invalid_subgroup_test() {
        match unfold("gt:feraxhp///test") {
            Ok(res)  => {
                panic!("Expected to fail, got {:?}", res);
            }
            Err(_) => (),
        }
    }
    
    #[test]
    fn gt_default_owner_test() {
        match unfold("gt:*/prueba") {
            Ok((Some(pconf), owner, repo_name)) => {
                assert_eq!(pconf, "gt");
                assert_eq!(owner, "gtuser");
                assert_eq!(repo_name, "prueba");
            },
            Ok(res)  => {
                panic!("Expected (Some(pconf), owner, repo_name), got {:?}", res);
            }
            Err(e) => panic!("Err: {}", e),
        }
    }
}