
use std::{env, ffi::OsStr};
use clap_complete::CompletionCandidate;
use git2::BranchType;

use super::super::structure::Completer;


pub struct Branch;


impl<'a> Completer for Branch {
    fn canditates(current: &OsStr) -> Vec<CompletionCandidate> {
        let prefix = current.to_string_lossy();
        let path = match env::current_dir() {
            Ok(p) => p,
            Err(_) => return vec![],
        };
        
        let repo = match git2::Repository::discover(path) {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        
        let remotes = match repo.branches(Some(BranchType::Local)) {
            Ok(sa) => sa,
            Err(_) => return vec![],
        };
        
        remotes
            .filter_map(|b| {
                let branch = match b {
                    Ok(b) => b,
                    Err(_) => return None,
                };
                
                let name = match branch.0.name() {
                    Ok(Some(e)) => e,
                    _ => return None,
                };
                
                if prefix.is_empty() || name.starts_with(&*prefix) {
                    Some(CompletionCandidate::new(name))
                }
                else 
                { None }
            })
            .collect()
    }
}
