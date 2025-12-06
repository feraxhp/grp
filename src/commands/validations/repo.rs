
use std::fmt::Display;
use color_print::cformat;

use crate::girep::{error::structs::Error, platform::Platform};


#[derive(Debug, Clone)]
pub struct RepoStructure {
    pub pconf: Option<String>,
    pub owner: String,
    pub path: String,
    pub len: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MultipleConsecutiveSlashes,
    StartWithDobleDots,
    MultipleDobleDots,
    EmptyStructure,
    BadSpace,
    NoSlash,
    NoOwner,
    NoRepo,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = match &self {
            ParseError::MultipleConsecutiveSlashes => cformat!("* <g><<pconf>:<<owner>/<<repo>[/..]</> can not contain multiple consecutive <y>'/'</>"),
            ParseError::StartWithDobleDots =>  cformat!("* <g><<pconf><r>:</r><<owner>/<<repo>[/..]</> can not start with <y>':'</>"),
            ParseError::MultipleDobleDots =>  cformat!("* <g><<pconf><r>:</r><<owner>/<<repo>[/..]</> can not contain multiple <y>':'</>"),
            ParseError::EmptyStructure => cformat!("* <g><<pconf>:<r><<owner>/<<repo>[/..]</> the repo path can not be empty"),
            ParseError::BadSpace => cformat!("* <g><<pconf>:<<owner>/<<repo>[/..]</> can not contain any <y>space ' '</>"),
            ParseError::NoSlash => cformat!("* <g><<pconf>:<<owner><r>/<<repo>[/..]</> is missing the <m>repo</>"),
            ParseError::NoOwner => cformat!("* <g><<pconf>:<r><<owner></r>/<<repo>[/..]</> is missing the <m>owner</>"),
            ParseError::NoRepo => cformat!("* <g><<pconf>:<<owner>/<r><<repo>[/..]</> is missing the <m>repo</>"),
        };
        
        write!(f, "{}", message)
    }
}

#[allow(dead_code, unused_variables)]
impl RepoStructure {
    pub fn parse<S: Into<String>>(value: S) -> Result<Self, ParseError> {
        let input = value.into();
        
        if input.starts_with(":") { return Err(ParseError::StartWithDobleDots) }
        if input.contains(" ") { return Err(ParseError::BadSpace) }
        if input.contains("//") { return Err(ParseError::MultipleConsecutiveSlashes) }
        
        let mut intermediate: String = input.clone();
        let mut pconf: Option<String> = None;
        
        if input.contains(":") {
            let parts: Vec<String> = input.split(":").map(|s| s.to_string()).collect();
            if parts.len() > 2 { return Err(ParseError::MultipleDobleDots); }
            if parts.len() < 2 { unreachable!() }
            
            pconf = match parts[0].clone() {
                s if s.eq("*") => None,
                s => Some(s)
            };
            intermediate = parts[1].clone();
        }
        
        if intermediate.is_empty() { return Err(ParseError::EmptyStructure) }
        if ! intermediate.contains("/") { return Err(ParseError::NoSlash) }
        
        let mut parts: Vec<String> = intermediate.split("/").map(|s| s.to_string()).collect();
        
        let len: usize = parts.len();
        let repo: String = parts.pop().unwrap_or(String::new());
        let owner: String = parts.join("/");
        
        if owner.is_empty() || owner.starts_with("/") { return Err(ParseError::NoOwner); }
        if repo.is_empty() { return Err(ParseError::NoRepo); }
        
        Ok(Self {
            pconf: pconf, 
            owner: owner, 
            path: repo, 
            len: (len - 1) 
        })
    }
    
    pub fn is_supported(&self, platform: &Platform) -> bool { self.len <= platform.max_repo_depth() as usize }
    pub fn is_unsupported(&self, platform: &Platform) -> Result<(), Error> { 
        if ! self.is_supported(platform) {
            Err(Error::new_custom(
                "The repo path is longer than the supported length".to_string(), 
                vec![
                    cformat!("* <m>{}</> does not suport repos of lenght <r>{}</>", platform.name(), self.len)
                ]
            ))
        } else { Ok(()) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_structure() {
        let r = RepoStructure::parse("pconf:owner/repo_name").unwrap();
        assert_eq!(r.pconf, Some("pconf".to_string()));
        assert_eq!(r.owner, "owner".to_string());
        assert_eq!(r.path, "repo_name".to_string());
        assert_eq!(r.len, 1)
    }
    
    #[test]
    fn perfect_structure_multiple() {
        let r = RepoStructure::parse("pconf:owner/repo_name/nested/on_another").unwrap();
        assert_eq!(r.pconf, Some("pconf".to_string()));
        assert_eq!(r.owner, "owner/repo_name/nested".to_string());
        assert_eq!(r.path, "on_another".to_string());
        assert_eq!(r.len, 3)
    }
    
    #[test]
    fn space_in_structure() {
        let r = RepoStructure::parse("pconf:owner/repo name/nested/on_another").unwrap_err();
        assert_eq!(r, ParseError::BadSpace)
    }
    
    #[test]
    fn no_owner() {
        let r = RepoStructure::parse("pconf:/repo_name/nested/on_another").unwrap_err();
        assert_eq!(r, ParseError::NoOwner)
    }
    
    #[test]
    fn no_repo() {
        let r = RepoStructure::parse("pconf:owner/").unwrap_err();
        assert_eq!(r, ParseError::NoRepo)
    }
    
    #[test]
    fn no_doble_slashes() {
        let r = RepoStructure::parse("pconf:owner//repo_name").unwrap_err();
        assert_eq!(r, ParseError::MultipleConsecutiveSlashes)
    }
    
    #[test]
    fn no_triple_slashes() {
        let r = RepoStructure::parse("pconf:owner///repo_name").unwrap_err();
        assert_eq!(r, ParseError::MultipleConsecutiveSlashes)
    }
    
    #[test]
    fn no_pconf() {
        let r = RepoStructure::parse("owner/repo_name").unwrap();
        assert_eq!(r.pconf, None);
        assert_eq!(r.owner, "owner".to_string());
        assert_eq!(r.path, "repo_name".to_string());
        assert_eq!(r.len, 1)
    }
    
    #[test]
    fn asteric_pconf() {
        let r = RepoStructure::parse("*:owner/repo_name").unwrap();
        assert_eq!(r.pconf, None);
        assert_eq!(r.owner, "owner".to_string());
        assert_eq!(r.path, "repo_name".to_string());
        assert_eq!(r.len, 1)
    }
    
    #[test]
    fn start_with_doble_dots() {
        let r = RepoStructure::parse(":owner/repo_name").unwrap_err();
        assert_eq!(r, ParseError::StartWithDobleDots)
    }
}
