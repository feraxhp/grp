use grp_core::location;
use git2::{Error, ErrorClass, ErrorCode, Repository};

use super::super::structs::GitUtils;
use super::super::options::Methods;

impl Methods {
    pub fn get_push_refs(&self, repo: &Repository, branch: Option<&String>, force: &bool) -> Result<Vec<String>, Error> {
        let force_str = if *force { "+" } else { "" };
        let vec = match self {
            Methods::All => {
                let mut refs = Self::Branches.get_push_refs(repo, branch, force)?;
                refs.extend(Self::Tags.get_push_refs(repo, branch, force)?);
                refs
            },
            Methods::Tag(tag) => {
                vec![Self::tag_ref(force_str, tag)]
            }
            Methods::Tags => {
                GitUtils::list_tags(repo)
                    .into_iter()
                    .map(|branch| Self::tag_ref_formated(force_str, &branch))
                    .collect()
            },
            Methods::Branches => {
                GitUtils::list_branches(repo)?
                    .into_iter()
                    .map(|branch| Self::branch_ref(force_str, &branch))
                    .collect()
            },
            Methods::Default |
            Methods::Upstream => {
                vec![
                    Self::branch_ref(
                        force_str, 
                        branch.ok_or(
                            Error::new(
                                ErrorCode::Invalid, 
                                ErrorClass::Reference, 
                                format!("location:{}", location!())
                            )
                        )?
                    )
                ]
            },
        };
        
        Ok(vec.clone())
    }
}