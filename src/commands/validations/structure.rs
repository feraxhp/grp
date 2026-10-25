
use color_print::cformat;

use crate::commands::validations::repo::RepoStructure;

#[allow(dead_code)]
pub struct Raw<T>(T);
pub trait Validations {
    type Output;
    
    fn value_parcer(value: &str) -> Result<Self::Output, String>;
    fn strict_value_parcer(value: &str) -> Result<Self::Output, String>;
}

impl Validations for RepoStructure {
    type Output = RepoStructure;
    
    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        match Self::parse(value) {
            Ok(repo) => Ok(repo),
            Err(e) => Err(format!("\n{}", e)),
        }
    }
    
    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        match Self::parse(value) {
            Ok(repo) => {
                if repo.pconf.is_none() { Err(cformat!("\n * The <m>pconf</> is <i,r>obligatory</> for <b>security reasons</>: <g><r><<pconf></r>:<<owner>/<<repo>[/..]</> ")) }
                else { Ok(repo) }
            }
            Err(e) => Err(format!("\n{}", e)),
        }
    }
}
