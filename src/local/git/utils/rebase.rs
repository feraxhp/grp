use git2::{AnnotatedCommit, Error, ErrorClass, ErrorCode, ResetType, Repository};
use std::process::{Command, Stdio};
use std::path::Path;
use color_print::cformat;

use crate::local::git::structs::GitUtils;


impl GitUtils {
    pub(crate) fn rebase(
        repo: &Repository,
        local: &AnnotatedCommit,
        upstream: &AnnotatedCommit,
    ) -> Result<String, Error> {
        let branch = Self::get_branch_name(repo)?;
        
        let head_commit_oid = repo.head()?.target().ok_or_else(|| {
            Error::new(ErrorCode::UnbornBranch, ErrorClass::Reference, "HEAD is detached or branch is unborn")
        })?;

        let git2_result = Self::try_git2_rebase(repo, local, upstream);

        match git2_result {
            Ok(success_message) => Ok(success_message),
            Err(e) => {
                match (e.code(), e.class()) {
                    (ErrorCode::Unmerged, ErrorClass::Rebase) => {
                        let head_commit_obj = repo.find_commit(head_commit_oid)?.into_object();
                        repo.reset(&head_commit_obj, ResetType::Hard, None)?;

                        Self::rebase_with_git(repo, branch, upstream)
                    }
                    _ => Err(e)
                }
            }
        }
    }

    fn try_git2_rebase(
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

        while let Some(_operation) = rebase.next().transpose()? {
            match rebase.commit(None, &repo.signature()?, None) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        };

        rebase.finish(None)?;
        Ok(cformat!("<g>* <i><m>Rebase</m> completed successfully</>"))
    }

    fn rebase_with_git(
        repo: &Repository,
        branch: String,
        upstream: &AnnotatedCommit,
    ) -> Result<String, Error> {
        let remote_id = upstream.id().to_string();
        let workdir: &Path = repo.workdir().ok_or_else(|| {
            Error::new(ErrorCode::NotFound, ErrorClass::Repository, "Cannot find repository working directory")
        })?;
        let path = workdir.to_string_lossy().into_owned();

        let rebase = Command::new("git")
            .arg("-C")
            .arg(&path)
            .arg("rebase")
            .arg(remote_id)
            .arg(branch)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::new(
                ErrorCode::GenericError,
                ErrorClass::Os,
                format!("Failed to execute git command: {}", e)
            ))?;

        match rebase.status {
            s if s.success() => Ok(
                cformat!("<g>* <i><m>Rebase</m> completed successfully</>")
            ),
            _ => Err(
                rebase_error(
                    // local, upstream
                )
            )
        }
    }
}

fn rebase_error(
    // local: &AnnotatedCommit,
    // upstream: &AnnotatedCommit
) -> Error {
    // let local_id = local.id().to_string();
    // let upstream_id = upstream.id().to_string();

    Error::new(
        ErrorCode::Unmerged,
        ErrorClass::Rebase,
        "unstaged changes exist in workdir"
    )
}
