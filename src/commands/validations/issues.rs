use color_print::cformat;

use crate::commands::validations::structure::Validations;
use crate::commands::validations::repo::RepoStructure;


#[derive(Debug, Clone)]
pub struct IssueStructure {
    pub repo: RepoStructure,
    pub number: u64,
}

impl IssueStructure {
    fn from(repo: RepoStructure) -> Result<Self, String> {
        let path = repo.path;
        
        if !path.contains("#") {
            return Err(
                cformat!("\n<y>Invalid issue structure, you must specify a issue number</>\n  <g><<pconf>:<<owner>/<<repo><r>#<<issue></>")
            );
        };
        
        let parts = path.split("#").collect::<Vec<&str>>();
        if parts.len() > 2 {
            return Err(
                cformat!("\n<y>Invalid issue structure, can only have one issue number</>\n  <g><<pconf>:<<owner>/<<repo><c>#<<issue></><r>#...</>")
            );
        }
        
        if parts.len() < 2 {
            return Err(
                cformat!("\n<y>Invalid issue structure, you must specify a valid number</>\n  <g><<pconf>:<<owner>/<<repo>#<r><<issue></>")
            );
        }
        
        let number = match parts[1].parse::<u64>() {
            Ok(v) => v,
            Err(e) => return Err(
                cformat!("\n<y>Invalid issue number</>: <r>{}</>\n  <g><<pconf>:<<owner>/<<repo>#<r><<issue></>", e)
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

    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        let repo = RepoStructure::value_parcer(value)?;
        IssueStructure::from(repo)
    }

    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        let repo = RepoStructure::strict_value_parcer(value)?;
        IssueStructure::from(repo)
    }
}
