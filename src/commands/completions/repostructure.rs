use std::ffi::OsStr;

use clap_complete::{ArgValueCompleter, CompletionCandidate};

use crate::girep::usettings::structs::Usettings;

use super::structure::Completer;
use super::super::validations::repo::RepoStructure;

impl Completer for RepoStructure {
    fn complete() -> ArgValueCompleter { ArgValueCompleter::new(Self::canditates) }

    fn canditates(current: &OsStr) -> Vec<CompletionCandidate> {
        let prefix = current.to_string_lossy();
        let is_simple = prefix.starts_with(".");
        
        let candidates: Vec<String> = match Usettings::read() {
            Ok(u) if u.pconfs.len() > 0 => u.pconfs
                .iter()
                .filter_map(|p| {
                    let compl = 
                        if p.owner.is_empty() || is_simple 
                        { format!("{}:", p.name) }
                        else 
                        { format!("{}:{}/", p.name, p.owner) };
                    
                    let dot_compl = format!(".{compl}");
                    
                    if 
                        prefix.is_empty() || 
                        compl.starts_with(&*prefix) ||
                        ( is_simple && dot_compl.starts_with(&*prefix) )
                    { Some(compl) } 
                    else 
                    { None }
                })
                .collect()
            ,
            _ => vec![],
        };
        
        candidates
            .iter()
            .map(|c| {
                if 
                    is_simple &&
                    candidates.len() > 1
                { CompletionCandidate::new(c).add_prefix(".") }
                else 
                { CompletionCandidate::new(c) }
            })
            .collect()
    }
}
