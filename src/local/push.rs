use std::sync::Arc;
use std::sync::Mutex;
use std::path::PathBuf;
use color_print::cformat;
use git2::{Error, ErrorClass, ErrorCode, PushOptions, Repository};

use crate::girep::usettings::structs::Pconf;
use crate::girep::usettings::structs::Usettings;
use crate::girep::animation::Animation;
use crate::girep::platform::Platform;

use super::git::options::{Methods, Options};
use super::git::structs::GitUtils;


impl Platform {
    /// return: __logs__, true (_no errors on push_) - false (_some errors on push_)
    pub(crate) fn push_repo<A: Animation + ?Sized>(
        path: &PathBuf,
        pconf: Option<Pconf>,
        options: Options,
        usettings: &Usettings,
        animation: &Box<A>
    ) -> Result<(Vec<String>, bool), Error> {
        animation.change_message("Getting the local repository ...");
        let repo = Repository::discover(path)?;
        
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
        let ref_specs= options.method
            .get_push_refs(&repo, Some(&branch_name), &options.force)?;
        
        if ref_specs.is_empty() {
            return Ok((vec![cformat!("<g>*</> <r,i>Nothing</> <i>to do!</>")], true));
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

            logs.push(cformat!("{}. Push <y>{}</> refs to <m>{}</>", step_count, ref_specs.len(), &remote_name));
            for reff in ref_specs {
                let reff = match options.force {
                    true  => cformat!("<r>+</><y>{}</>", &reff[1..]),
                    false => cformat!("<y>{}</>", reff)
                };
                logs.push(cformat!("  <g>»</> {}", reff));
            }
            
            return Ok((logs, true))
        }
        
        animation.change_message("Setting up credentials ...");
        let mut callbacks = GitUtils::get_credential_callbacks(&config);
        
        let logs = Arc::new(Mutex::new(Vec::new()));
        let perfect = Arc::new(Mutex::new(true));
        let transfer = Arc::new(Mutex::new(0));
        
        let logs_clone = logs.clone();
        let perfect_clone = perfect.clone();
        let transfer_clone = transfer.clone();
        
        callbacks.push_update_reference(move |refs, status| {
            let mut logs = logs_clone.lock().unwrap();
            let mut perfect = perfect_clone.lock().unwrap();
            let transfer = transfer_clone.lock().unwrap();
            
            if let Some(error) = status {
                let message = cformat!("<r>* <m>{}</><w> got <r>Error:</> <i>{}</>", refs, &error);
                // animation.change_message(&message);
                logs.push(message);
                *perfect = false;
                return Ok(());
            }
            
            let message = match (*transfer, options.force) {
                (0, false) => cformat!("<g>* <m>{}</><w> is <g>Up-to-date!</>", refs),
                (_, true)  => cformat!("<g>* <r>+<m>{}</><w> was <r>force</r> <g>pushed</>", refs),
                _          => cformat!("<g>* <m>{}</><w> was <g>pushed</>", refs)
            };
            
            // animation.change_message(&message);
            logs.push(message);
            
            Ok(())
        });
        
        let transfer_clone = transfer.clone();

        callbacks.push_transfer_progress(move |current, total, bytes| {
            let mut transfer = transfer_clone.lock().unwrap(); 
            *transfer = total;
            
            let message = cformat!("Progress: {}/{} objects transferred ({} bytes)", current, total, bytes); 
            animation.change_message(&message); 
        });
        
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        animation.change_message("Pushing repository ...");
        let mut remote = repo.find_remote(&remote_name)?;
        remote.push(&ref_specs, Some(&mut push_options))?;
        
        let final_logs = logs.lock().unwrap().clone();
        let final_perfect = perfect.lock().unwrap().clone();

        Ok((final_logs, final_perfect))
    }
}