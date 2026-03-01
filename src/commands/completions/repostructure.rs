use std::ffi::OsStr;

use clap_complete::CompletionCandidate;
use grp_core::structs::Repo;

use crate::cache::structure::Cacher;
use crate::candiates;
use crate::usettings::structs::Usettings;
use super::structure::Completer;
use super::super::validations::repo::RepoStructure;

type VRepo = Vec<Repo>;
impl Completer for RepoStructure {
    candiates!();
    
    fn ccanditates(current: &OsStr) -> Vec<CompletionCandidate> {
        let prefix = current.to_string_lossy();        
        let is_simple = prefix.starts_with(".");
        let is_complete_pconf = prefix.contains(":");
        
        match (is_simple, is_complete_pconf) {
            (true, true) => vec![],
            (is_simple, false) => {
                let real_current: String = if is_simple {
                    prefix.chars().skip(1).collect()
                } else {
                    prefix.to_string()
                };
                
                let candidates = Usettings::canditates(&OsStr::new(real_current.as_str()));
                let candidates_len = candidates.len();
                candidates
                    .into_iter()
                    .flat_map(|c| {
                        if !c.starts_with(&*prefix) { return None }
                        let mut candidate = CompletionCandidate::new(format!("{c}:"));
                        
                        if candidates_len != 1 && is_simple { 
                            candidate = candidate.add_prefix(".");
                        }
                        
                        Some(candidate)
                    })
                    .collect()
            },
            (false, true) => {
                let parts: Vec<&str> = prefix.split(':').collect();
                
                if parts.len() > 2 { return vec![] }
                
                let pconf = *parts.get(0).unwrap();
                let body = *parts.get(1).unwrap_or(&"");
                
                let pconf_owner = match Usettings::read() {
                    Ok(us) => us
                        .get_pconf_by_name(pconf)
                        .map(|p| format!("{}/", p.owner))
                        .unwrap_or_default(),
                    Err(_) => todo!(),
                };
                
                let repos = match VRepo::get(pconf) {
                    Ok(mut r) => { let _ = r.insert(pconf_owner); r },
                    Err(_) => return vec![],
                };
                
                repos.iter()
                    .flat_map(|s| {
                        if !s.starts_with(body) { return None }
                        let candidate = format!("{pconf}:{s}");
                        Some(CompletionCandidate::new(candidate))
                    })
                    .collect()
            },
        }
    }
}
