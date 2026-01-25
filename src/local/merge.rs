use color_print::cformat;
use git2::build::CheckoutBuilder;
use git2::{AnnotatedCommit, Error, Repository};
use grp_core::animation::Animation;

use crate::local::git::structs::GitUtils;
use crate::local::structs::Local;

impl Local {
    /// > Result: Message, perfect (true: no errors)
    pub fn merge_fetch<'repo, A: Animation + ?Sized>(
        repo: &'repo Repository,
        remote_branch_name: &str,
        fetch_commit: AnnotatedCommit<'repo>,
        force: bool,
        rebase: bool,
        animation: &Box<A>
    ) -> Result<(String, bool), Error> {
        
        animation.change_message("Performing merge analysis ...");
        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        match analysis.0 {
            a if a.is_fast_forward() => {
                animation.change_message("Fast-forward operation detected ...");
                let refname = format!("refs/heads/{}", remote_branch_name);

                match repo.find_reference(&refname) {
                    Ok(mut r) => {
                        animation.change_message("Performing fast-forward merge ...");
                        Ok((GitUtils::fast_forward(repo, &mut r, &fetch_commit, force)?, true))
                    }
                    Err(_) => {
                        let m = cformat!("<m>Setting:</> <y>{}</> to <m>{}</>", remote_branch_name, fetch_commit.id());
                        animation.change_message(m);
                        
                        repo.reference(&refname, fetch_commit.id(), true, "")?;
                        repo.set_head(&refname)?;

                        let mut builder = CheckoutBuilder::default();

                        builder.allow_conflicts(true);
                        builder.conflict_style_merge(true);
                        builder.force();

                        repo.checkout_head(Some(&mut builder))?;

                        Ok((cformat!("<g>* New branch created</> <m>{}</> ", remote_branch_name), true))
                    }
                }
            },

            a if a.is_normal() => {
                animation.change_message("Performing normal merge ...");
                let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
                if !rebase { Ok((GitUtils::merge(&repo, &head_commit, &fetch_commit)?, true)) }
                else { Ok((GitUtils::rebase(&repo, &head_commit, &fetch_commit)?, true)) } 
            },

            a if a.is_unborn() => {
                animation.change_message("Empty repoistory detected ...");
                let refname = format!("refs/heads/{}", remote_branch_name);

                repo.reference(&refname, fetch_commit.id(), true, "")?;
                repo.set_head(&refname)?;

                repo.checkout_head(Some(CheckoutBuilder::default().force(),))?;

                Ok((cformat!("<g>* Initialized repository</>"), true))
            },

            a if a.is_up_to_date() => {
                let refs = format!("refs/heads/{}", remote_branch_name);
                Ok((cformat!("<g>* <m>{}</><w> is <g>Up-to-date!</>", refs), true))
            },

            a => {
                // For debugging or logging purposes
                let analysis_description = format!(
                    "fast_forward: {}, normal: {}, up_to_date: {}, unborn: {}",
                    a.is_fast_forward(), a.is_normal(), a.is_up_to_date(), a.is_unborn()
                );
                
                Ok((cformat!("No action taken. Merge analysis: {}", analysis_description), true))
            }
        }
    }
}