
use color_print::cformat;

use crate::commands::validations::repo::RepoStructure;


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
                if None == repo.pconf { Err(cformat!("\n * The <m>pconf</> is <i,r>obligatory</>: <g><r><<pconf></r>:<<owner>/<r><<repo>[/..]</> ")) }
                else { Ok(repo) }
            }
            Err(e) => Err(format!("\n{}", e)),
        }
    }
}