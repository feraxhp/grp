use color_print::cformat;
use git2::{AnnotatedCommit, AutotagOption, Error, ErrorClass, ErrorCode, FetchOptions, Repository};

use crate::usettings::structs::{Pconf, Usettings};
use super::git::structs::GitUtils;
use super::git::options::{Methods, Options};
use crate::girep::platform::Platform;
use crate::girep::animation::Animation;

pub struct FetchResult<'repo> {
    pub id: Option<AnnotatedCommit<'repo>>,
    pub logs: Vec<String>,
    pub branch: String,
} 

impl Platform {
    pub(crate) fn fetch_repo<'repo, A: Animation + ?Sized>(
        repo: &'repo Repository,
        pconf: Option<Pconf>, 
        options: Options,
        usettings: &Usettings, 
        animation: Option<&Box<A>>
    ) -> Result<FetchResult<'repo>, Error> {
        match options.method {
            Methods::ALL      |
            Methods::TAG(_)   |
            Methods::TAGS     |
            Methods::BRANCHES => {
                return Err(
                    Error::new(ErrorCode::Invalid, ErrorClass::Invalid, "Method not allowed for fetch")
                );
            }
            Methods::DEFAULT |
            Methods::UPSTREAM => (),
        };
        
        if let Some(an) = animation { an.change_message("Getting the branch and remote ..."); }
        let (
            branch_name,
            remote_name
        ) = GitUtils::get_repo_branch_and_remote(&repo, &options)?;
        
        if options.method == Methods::UPSTREAM && !options.dry_run.clone() {
            if let Some(an) = animation { an.change_message("Setting upstream ..."); }
            let _ = options.method.set_upstream(&repo, &branch_name, &remote_name)?;
        };
        
        if let Some(an) = animation { an.change_message("Preparing ref_specs ..."); }
        let mut remote = repo.find_remote(&remote_name)?;
        let ref_specs= options.method
            .get_fetch_refs(&branch_name, &remote)?;
        
        if ref_specs.is_empty() {
            let fr = FetchResult {
                id: None,
                logs: vec![cformat!("<g>*</> <r,i>Nothing</> <i>to do!</>")],
                branch: branch_name.clone()
            };
            return Ok(fr);
        };
        
        let pconf = match pconf {
            Some(e) => e,
            None => usettings
                    .get_pconf_or_default(&remote_name)
                    .ok_or(Error::new(ErrorCode::NotFound, ErrorClass::Config, "no pconf"))?
        };
        
        let config = pconf.to_config();
        
        if options.dry_run {
            
            let mut logs = vec![
                cformat!("1. Unsing <i>pconf</> <m>{}</>", &config.pconf)
            ];
            
            let mut step_count: u8 = 2;
            if matches!(options.method, Methods::UPSTREAM) {
                logs.push(cformat!("{}. Set upstream", step_count));
                logs.push(cformat!("  <g>⁕</> <y>{}</> to branch: <y>{}</>", &remote_name, branch_name));
                step_count += 1;
            }

            logs.push(cformat!("{}. Pull <y>{}</> refs from <m>{}</>", step_count, ref_specs.len(), &remote_name));
            for reff in ref_specs {
                let reff = match options.force {
                    true  => cformat!("<r>+</><y>{}</>", &reff[1..]),
                    false => cformat!("<y>{}</>", reff)
                };
                logs.push(cformat!("  <g>»</> {}", reff));
            }
            
            let fr = FetchResult {
                id: None,
                logs: logs,
                branch: branch_name.clone()
            };
            return Ok(fr)
        }
        
        if let Some(an) = animation { an.change_message("Setting up credentials ..."); }
        let mut callbacks = GitUtils::get_credential_callbacks(&config);
        
        if let Some(an) = animation {
            callbacks.transfer_progress(|stats| {
                let message = if stats.total_objects() == 0 { return true; } 
                else if stats.received_objects() == stats.total_objects() {
                    format!(
                        "Resolving deltas {}/{}",
                        stats.indexed_deltas(),
                        stats.total_deltas()
                    )
                } 
                else {
                    format!(
                        "Received {}/{} objects ({}) in {} bytes",
                        stats.received_objects(),
                        stats.total_objects(),
                        stats.indexed_objects(),
                        stats.received_bytes()
                    )
                };
                
                an.change_message(message);
                true
            });
        }
        
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        fetch_options.download_tags(AutotagOption::Auto);
        
        let bbranches = GitUtils::get_branches_by_remote(repo, &remote_name)?;
        
        if let Some(an) = animation { an.change_message("Fetching repository ..."); }
        let _ = remote.fetch(&ref_specs, Some(&mut fetch_options), None)?;
        
        let abranches = GitUtils::get_branches_by_remote(repo, &remote_name)?;
        
        let mut logs = Vec::new();
        abranches.iter().filter(|(name, _)| !bbranches.contains_key(name.as_str()))
            .for_each(|(name, _)| { logs.push(cformat!("<g>* new branch</> <y>{}/{}</>", &remote_name, name)); });
        
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        
        let result = FetchResult {
            id: Some(repo.reference_to_annotated_commit(&fetch_head)?),
            logs: logs,
            branch: branch_name.clone(),
        };
        
        return Ok(result);
    }
}

