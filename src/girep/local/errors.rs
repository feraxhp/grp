use std::path::PathBuf;
use color_print::{cformat, cprintln};
use git2::{ErrorClass, ErrorCode};
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::config::Config;

impl Error {
    pub(crate) fn git_to_local_mapper(
        repo: PathBuf,
        pconf: Config,
    ) -> Box<dyn Fn(git2::Error) -> Error> {
        Box::new(move |e: git2::Error| {
            Self::git_to_local(e, repo.clone(), pconf.clone())
        })
    }

    pub(crate) fn git_to_local(error: git2::Error, repo: PathBuf, pconf: Config) -> Error {
        let code = error.code();
        let class_ = error.class();
        let path_str = repo.as_os_str().to_str().unwrap_or("{{ Break path }}");
        match (code, class_) {
            (ErrorCode::NotFound, ErrorClass::Repository) => {
                Error::new(
                    ErrorType::NotFoundRepo,
                    vec![
                        path_str,
                        cformat!("* Repository at: <m,i>{}</>", path_str).as_str(),
                        cformat!("  You may need to start a new repo: ").as_str(),
                        cformat!("  •<g> git init </>").as_str(),
                    ]
                )
            }
            (ErrorCode::NotFound, ErrorClass::Config) => {
                Error::new_custom(
                    "No remote found".to_string(),
                    vec![
                        cformat!("* Repository at: <m,i>{}</>", path_str),
                        cformat!("  You can add a remote by running the command: "),
                        cformat!("  •<g> git remote add origin <<url> </>"),
                        cformat!("  Or you can create a new remote with: "),
                        cformat!("  •<g> grp create -r={} {}:{}/{}</>", path_str, pconf.pconf, pconf.user, repo.file_name().unwrap_or_default().to_str().unwrap_or("<repo>")),
                    ]
                )
            }
            (ErrorCode::Auth, _) => {
                Error::new(
                    ErrorType::BadTokenScope,
                    vec![
                        pconf.pconf.as_str(),
                        "read/write repo",
                    ]
                )
            }
            (ErrorCode::Exists, _) => {
                let mut content: Vec<String> = Vec::new();
                if error.message().contains(" exists and is not an empty directory") {
                    let path = error.message().replace("'", "");
                    let path = path.replace(" exists and is not an empty directory", "");
                    content.append(
                        &mut vec![
                            cformat!("* The given directory is not <i>empty</>"),
                            "  Given path:".to_string(),
                            cformat!("    - <b,i,u>{}</>", path)
                        ]
                    )
                } else {
                    content.push(error.to_string())
                }
                Error::new_custom(
                    ErrorType::AlreadyExists.get_message(),
                    content
                )
            }
            (ErrorCode::GenericError, ErrorClass::Reference) => {
                Error::new(
                    ErrorType::NotFoundRepo,
                    vec![
                        path_str,
                    ]
                )
            }
            (ErrorCode::GenericError, ErrorClass::Http) => {
                match error.message() {
                    "too many redirects or authentication replays" => {
                        Error::new_custom(
                            ErrorType::Unauthorized.get_message(),
                            vec![
                                cformat!("<y>* The pconf may not be correct for the remote</>"),
                                cformat!("  <g>» Pconf : <m>{}</>", pconf.pconf),
                                cformat!("  <g>» Target: <m>{}</>", pconf.endpoint)
                            ]
                        )
                    }
                    message => {
                        Error::new_custom(
                            "Remote not found!".to_string(),
                            vec![
                                cformat!("* <y>The remote provided exist?</>"),
                                cformat!("  Verify the url of the remote you provide"),
                                cformat!("  You can do so by running the command: "),
                                cformat!("  •<g> git remote -v</>"),
                                cformat!("  And visiting the web page"),
                                cformat!("* <y>Push error response</>"),
                                cformat!("  - <m>{}</>", message),
                            ]
                        )
                    }
                }
            }
            (ErrorCode::UnbornBranch, _) => {
                Error::new_custom(
                    "Empty branch!".to_string(),
                    vec![
                        cformat!("* Repository at: <m,i>({})</>", path_str),
                        cformat!("  <r>The current branch has no commits</>")
                    ]
                )
            }
            (ErrorCode::NotFastForward, ErrorClass::Reference) => {
                Error::new_custom(
                    "The branch conflicts with the remote!".to_string(),
                    vec![
                        cformat!("* Repository at: <m,i>({})</>", path_str),
                        cformat!("  <r>{}</>", error.message())
                    ]
                )
            }
            _ => {
                Error::new(
                    ErrorType::Unknown,
                    vec![
                        cformat!("Repository: <m,i>({})</>", path_str).as_str(),
                        cformat!("  * Class: <m,i>({:?})</>", class_).as_str(),
                        cformat!("  * Code: <m,i>({:?})</>", code).as_str(),
                        error.message(),
                    ]
                )
            }
            // ErrorCode::Ambiguous |
            // ErrorCode::BufSize |
            // ErrorCode::User |
            // ErrorCode::BareRepo |
            // ErrorCode::Unmerged |
            // ErrorCode::InvalidSpec |
            // ErrorCode::Conflict |
            // ErrorCode::Locked |
            // ErrorCode::Modified |
            // ErrorCode::Certificate |
            // ErrorCode::Applied |
            // ErrorCode::Peel |
            // ErrorCode::Eof |
            // ErrorCode::Invalid |
            // ErrorCode::Uncommitted |
            // ErrorCode::Directory |
            // ErrorCode::MergeConflict |
            // ErrorCode::HashsumMismatch |
            // ErrorCode::IndexDirty |
            // ErrorCode::ApplyFail |
            // ErrorCode::Owner |
            // ErrorCode::Timeout |
        }
    }
}

