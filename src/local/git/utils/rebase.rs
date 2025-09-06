use git2::{AnnotatedCommit, Error, ErrorClass, ErrorCode, RebaseOperationType, Repository};
use color_print::cformat;

use crate::local::git::structs::GitUtils;


impl GitUtils {
    pub(crate) fn rebase(
        repo: &Repository,
        local: &AnnotatedCommit,
        upstream: &AnnotatedCommit,
    ) -> Result<String, Error> {
        let mut rebase_options = git2::RebaseOptions::new();
        let mut rebase = repo.rebase(
            Some(&local),
            Some(&upstream),
            None,
            Some(&mut rebase_options),
        )?;
        
        while let Some(operation) = rebase.next().transpose()? {
            match operation.kind() {
                Some(RebaseOperationType::Pick) => {
                    // Try to apply the commit
                    match rebase.commit(None, &repo.signature()?, None) {
                        Ok(_) => {}
                        Err(e) 
                        if e.code() == ErrorCode::Conflict => return Err(rebase_error(local, upstream)),
                        Err(e) => return Err(e),
                    }
                }
                Some(_) => {
                    // Handle other operation types normally
                    match rebase.commit(None, &repo.signature()?, None) {
                        Ok(_) => {}
                        Err(e) 
                        if e.code() == ErrorCode::Conflict => return Err(rebase_error(local, upstream)),
                        Err(e) => return Err(e),
                    }
                }
                None => {}
            };
        };

        rebase.finish(None)?;
        Ok(cformat!("<g>*</> <m>Rebase completed successfully</>"))
    }
}

fn rebase_error(local: &AnnotatedCommit, upstream: &AnnotatedCommit) -> Error {
    let local_id = local.id().to_string();
    let upstream_id = upstream.id().to_string();
    
    Error::new(
        ErrorCode::Conflict, 
        ErrorClass::Rebase,
        format!("r:{} l:{}", local_id, upstream_id)
    )
}