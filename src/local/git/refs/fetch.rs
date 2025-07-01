use git2::{Error, ErrorClass, ErrorCode, Remote};

use crate::local::git::options::Methods;

impl Methods {
    pub fn get_fetch_refs(&self, _branch: &String, remote: &Remote) -> Result<Vec<String>, Error> {
        let vec = match self {
            Methods::ALL      |
            Methods::TAG(_)   |
            Methods::TAGS     |
            Methods::BRANCHES => {
                return Err(
                    Error::new(ErrorCode::Invalid, ErrorClass::Invalid, "Method not allowed for fetch")
                );
            }
            Methods::DEFAULT |
            Methods::UPSTREAM  => {
                let refs = remote.refspecs();
                let mut specs: Vec<String> = Vec::new();
                for i in refs {
                    match (i.src(), i.dst()) {
                        (Some(src), Some(dst)) => {
                            specs.push(format!("{}:{}", src, dst))
                        }
                        (Some(src), _) if i.is_force() => {
                            specs.push(format!("+{}", src));
                        }
                        (Some(src), _) => {
                            specs.push(src.to_string());
                        }
                        _ => ()
                    }
                }
                specs
            }
        };
        
        Ok(vec.clone())
    }
}