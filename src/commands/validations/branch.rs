use color_print::cformat;

use crate::commands::validations::structure::Validations;
use crate::commands::validations::repo::RepoStructure;

const DIVIDER: &str = "?";

#[derive(Debug, Clone)]
pub struct BranchStructure {
    pub repo: RepoStructure,
    pub branch: String,
}

impl BranchStructure {
    fn from(repo: RepoStructure) -> Result<Self, String> {
        let path = repo.path;
        
        if !path.contains(DIVIDER) {
            return Err(
                cformat!("\n<y>Invalid branch structure, you must specify the branch name</>\n  <g><<pconf>:<<owner>/<<repo><r>{DIVIDER}<<branch></>")
            );
        };
        
        let parts = path.split(DIVIDER).collect::<Vec<&str>>();
        if parts.len() > 2 {
            return Err(
                cformat!("\n<y>Invalid branch structure, can only have one branch name</>\n  <g><<pconf>:<<owner>/<<repo><c>{DIVIDER}<<branch></><r>{DIVIDER}...</>")
            );
        }
        
        if parts.len() < 2 {
            return Err(
                cformat!("\n<y>Invalid branch structure, you must specify a valid name</>\n  <g><<pconf>:<<owner>/<<repo>{DIVIDER}<r><<branch></>")
            );
        }
        
        let branch = parts[1].to_string();
        if branch.is_empty() {
            return Err(
                cformat!("\n<y>Invalid branch structure, the name can not be <i>empty</>\n  <g><<pconf>:<<owner>/<<repo>{DIVIDER}<r><<branch></>")
            );
        }
        
        Ok(
            BranchStructure {
                repo: RepoStructure { 
                    pconf: repo.pconf.to_owned(),
                    owner: repo.owner.to_owned(),
                    path: parts[0].to_string(),
                    len: repo.len
                },
                branch
            }
        )
    }
}


impl Validations for BranchStructure {
    type Output = Self;

    fn value_parcer(_: &str) -> Result<Self::Output, String> {
        unimplemented!("No need to implement this function")
    }

    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        let repo = match RepoStructure::parse_skipping(value, Some(DIVIDER)) {
            Ok(repo) => {
                if repo.pconf.is_none() { Err(cformat!("\n * The <m>pconf</> is <i,r>obligatory</> for <b>security reasons</>: <g><r><<pconf></r>:<<owner>/<<repo>[/..]</> ")) }
                else { Ok(repo) }
            }
            Err(e) => Err(format!("\n{}", e)),
        }?;
        
        Self::from(repo)
    }
}
