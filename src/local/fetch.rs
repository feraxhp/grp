use std::path::PathBuf;

use color_print::cformat;
use git2::{AnnotatedCommit, AutotagOption, Error, ErrorClass, ErrorCode, FetchOptions, Repository};
use indicatif::HumanBytes;

use grp_core::animation::Animation;

use crate::local::structs::Local;
use super::git::structs::GitUtils;
use super::git::options::{Methods, Options};
use crate::animations::animation::Subprogress;
use crate::usettings::structs::{Pconf, Usettings};

pub struct FetchResult<'repo> {
    pub id: Option<AnnotatedCommit<'repo>>,
    pub logs: Vec<String>,
    pub branch: String,
} 

impl Local {
    pub(crate) fn fetch<'repo, A: Animation + Subprogress + ?Sized>(
        repo: &'repo Repository,
        pconf: Option<Pconf>, 
        options: Options,
        usettings: &Usettings, 
        animation: &mut Box<A>
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
        
        animation.change_message("Getting the branch and remote ...");
        let (
            branch_name,
            remote_name
        ) = GitUtils::get_repo_branch_and_remote(&repo, &options)?;
        
        if options.method == Methods::UPSTREAM && !options.dry_run.clone() {
            animation.change_message("Setting upstream ...");
            let _ = options.method.set_upstream(&repo, &branch_name, &remote_name)?;
        };
        
        animation.change_message("Preparing ref_specs ...");
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
        
        animation.change_message("Setting up credentials ...");
        let mut callbacks = GitUtils::get_credential_callbacks(&config);
        
        let _objects = animation.add();
        let _deltas = animation.add();
        
        callbacks.transfer_progress(|stats| {
            if stats.total_objects() == 0 { return true; } 
            else if stats.received_objects() == stats.total_objects() {
                animation.change_message("Resolving deltas ...");
                animation.set_state(1, stats.received_objects() as u64);
                animation.set_total(
                    2, stats.total_deltas() as u64, 
                    "    🔄 {percent:>3.blue}% {bar:30.green/blue}    {pos}/{len} on {elapsed_precise:.yellow}"
                );
                animation.set_state(2, stats.indexed_deltas() as u64);
            } 
            else {
                animation.change_message("Downloading objects ...");
                animation.set_total(
                    1, stats.total_objects() as u64, 
                    "    ⬇️ {percent:>3.blue}% {bar:30.green/blue}    {pos}/{len}: [{msg}] on {elapsed_precise:.yellow}"
                );
                animation.set_state(1, stats.received_objects() as u64);
                animation.set_message(1, format!("{} indexed: {}", HumanBytes(stats.received_bytes() as u64), stats.indexed_objects()));
            };
            
            true
        });
        
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        fetch_options.download_tags(AutotagOption::Auto);
        
        let bbranches = GitUtils::get_branches_by_remote(repo, &remote_name)?;
        
        animation.change_message("Fetching repository ...");
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
    
    pub async fn fetch_repo<A: Animation + Subprogress + ?Sized>(
        path: &PathBuf, 
        pconf: Option<Pconf>, 
        options: Options, 
        usettings: &Usettings, 
        animation: &mut Box<A>
    ) -> Result<Vec<String>, git2::Error> {
        let repo = Repository::discover(path)?;
        
        let result = Local::fetch(&repo, pconf, options, usettings, animation)?;
        
        Ok(result.logs)
    }
}
