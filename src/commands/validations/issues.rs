use color_print::cformat;

use crate::commands::validations::structure::Validations;
use crate::commands::validations::repo::RepoStructure;

const DIVIDER: &str = "#";

#[derive(Debug, Clone)]
pub struct IssueStructure {
    pub repo: RepoStructure,
    pub number: u64,
}

impl IssueStructure {
    fn from(repo: RepoStructure) -> Result<Self, String> {
        let path = repo.path;
        
        if !path.contains(DIVIDER) {
            return Err(
                cformat!("\n<y>Invalid issue structure, you must specify a issue number</>\n  <g><<pconf>:<<owner>/<<repo><r>{DIVIDER}<<issue></>")
            );
        };
        
        let parts = path.split(DIVIDER).collect::<Vec<&str>>();
        if parts.len() > 2 {
            return Err(
                cformat!("\n<y>Invalid issue structure, can only have one issue number</>\n  <g><<pconf>:<<owner>/<<repo><c>{DIVIDER}<<issue></><r>{DIVIDER}...</>")
            );
        }
        
        if parts.len() < 2 {
            return Err(
                cformat!("\n<y>Invalid issue structure, you must specify a valid number</>\n  <g><<pconf>:<<owner>/<<repo>{DIVIDER}<r><<issue></>")
            );
        }
        
        let number = match parts[1].parse::<u64>() {
            Ok(v) => v,
            Err(e) => return Err(
                cformat!("\n<y>Invalid issue number</>: <r>{}</>\n  <g><<pconf>:<<owner>/<<repo>{DIVIDER}<r><<issue></>", e)
            ),
        };

        Ok(
            IssueStructure {
                repo: RepoStructure { 
                    pconf: repo.pconf.to_owned(),
                    owner: repo.owner.to_owned(),
                    path: parts[0].to_string(),
                    len: repo.len
                },
                number
            }
        )
    }
}


impl Validations for IssueStructure {
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
