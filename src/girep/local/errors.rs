use std::path::PathBuf;
use color_print::cformat;
use git2::{ErrorClass, ErrorCode};
use crate::errors::error::Error;
use crate::errors::types::ErrorType;
use crate::girep::config::Config;

pub(crate) enum Action {
    PUSH,
    PULL,
    CLONE,
}

impl Action {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            Action::PUSH => "push",
            Action::PULL => "pull",
            Action::CLONE => "clone",
        }
    }
}

impl Error {
    pub(crate) fn git_to_local(error: git2::Error, repo: PathBuf, pconf: Config, action: Action) -> Error {
        let code = error.code();
        let class_ = error.class();
        let message = error.message().to_string();
        let path_str = repo.as_os_str().to_str().unwrap_or("{{ Break path }}");
        match (code, class_, message.as_str(), action) {
            (ErrorCode::NotFound, ErrorClass::Repository, _, _) => {
                Error::new(
                    ErrorType::NotFoundRepo,
                    vec![
                        path_str,
                        cformat!("  You may need to start a new repo: ").as_str(),
                        cformat!("  •<g> git init </>").as_str(),
                    ]
                )
            }
            (ErrorCode::NotFound, ErrorClass::Config, msg, action) if msg.starts_with("-u ") => {
                Error::new_custom(
                    "No upstream set".to_string(),
                    vec![
                        cformat!("<y>* The current branch has no <i>Upstream</> set"),
                        cformat!("  You can set it by running the command:"),
                        cformat!("  •<g> grp {} {}</>", action.to_str(), msg),
                    ]
                )
            }
            (ErrorCode::NotFound, ErrorClass::Config, _, _) => {
                Error::new_custom(
                    "No remote found".to_string(),
                    vec![
                        cformat!("<y>* Repository at: <m,i>{}</>", path_str),
                        cformat!("  You can add a remote by running the command: "),
                        cformat!("  •<g> git remote add origin <<url> </>"),
                        cformat!("  Or you can create a new remote with: "),
                        cformat!("  •<g> grp create -r={} {}:{}/{}</>", path_str, pconf.pconf, pconf.user, repo.file_name().unwrap_or_default().to_str().unwrap_or("<repo>")),
                    ]
                )
            }
            (ErrorCode::Auth, _, _, _) => {
                Error::new(
                    ErrorType::BadTokenScope,
                    vec![
                        pconf.pconf.as_str(),
                        "read/write repo",
                    ]
                )
            }
            (ErrorCode::Exists, _, _, _) => {
                let mut content: Vec<String> = Vec::new();
                if error.message().contains(" exists and is not an empty directory") {
                    let path = error.message().replace("'", "");
                    let path = path.replace(" exists and is not an empty directory", "");
                    content.append(
                        &mut vec![
                            cformat!("<y>* The given directory is not <m,i>empty</>"),
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
            (ErrorCode::GenericError, ErrorClass::Reference, _, _) => {
                Error::new(
                    ErrorType::NotFoundRepo,
                    vec![
                        path_str,
                    ]
                )
            }
            (ErrorCode::GenericError, ErrorClass::Http, _, action) => {
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
                    "request failed with status code: 404" if matches!(action, Action::PUSH) => {
                        Error::new_custom(
                            "Target remote URL does not exist!".to_string(),
                            vec![
                                cformat!("<y>* The remote URL provided is unreachable.</>"),
                                cformat!("  Please verify the URL of the remote by running:"),
                                cformat!("  • <g>git remote -v</>"),
                                cformat!("  Visit the web page of the remote repository to confirm."),
                                cformat!("  <g>Tip</>: If the repository doesn't exist at the <m,i>remote URL</>,"),
                                cformat!("       you may need to create it first."),
                            ]
                        )
                    }
                    "request failed with status code: 404" if matches!(action, Action::CLONE) => {
                        Error::new_custom(
                            "The repository does not exist!".to_string(),
                            vec![
                                cformat!("<y>* The URL provided is unreachable.</>")
                            ]
                        )
                    }
                    message => {
                        Error::new_custom(
                            "Remote not found!".to_string(),
                            vec![
                                cformat!("<y>* The remote provided exist?</>"),
                                cformat!("  Verify the url of the remote you provide"),
                                cformat!("  You can do so by running the command: "),
                                cformat!("  •<g> git remote -v</>"),
                                cformat!("  And visiting the web page"),
                                cformat!("<y>* Error message</>"),
                                cformat!("  - <m>{}</>", message),
                            ]
                        )
                    }
                }
            }
            (ErrorCode::UnbornBranch, _, _, _) => {
                Error::new_custom(
                    "Empty branch!".to_string(),
                    vec![
                        cformat!("<y>* Repository at: <m,i>({})</>", path_str),
                        cformat!("  <r>The current branch has no commits</>")
                    ]
                )
            }
            (ErrorCode::NotFastForward, ErrorClass::Reference, _, Action::PUSH) => {
                Error::new_custom(
                    "No fast-forward push".to_string(),
                    vec![
                        cformat!("<y>* The branch conflicts with the remote"),
                        cformat!("  you have to solved it fist"),
                        cformat!("  or <i,m>add the <r>--force</r> tag</>"),
                    ]
                )
            }
            (ErrorCode::NotFastForward, ErrorClass::Reference, _, _) => {
                Error::new_custom(
                    "The branch conflicts with the remote!".to_string(),
                    vec![
                        cformat!("<y>* Repository at: <m,i>({})</>", path_str),
                        cformat!("  <r>{}</>", error.message())
                    ]
                )
            }
            (ErrorCode::Conflict, ErrorClass::Merge, msg, Action::PULL) => {
                let msg = msg.split(",").collect::<Vec<&str>>();
                Error::new_custom(
                    "The remote conflicts with the branch!".to_string(),
                    vec![
                        cformat!("<g>* Tip:</>  <Y>You can fix this with normal git commands</>")
                    ]
                )
            }
            (ErrorCode::Locked, ErrorClass::Merge, msg, Action::PULL) => {
                let msg = msg.split(",").collect::<Vec<&str>>();
                Error::new_custom(
                    "Uncommitted changes".to_string(),
                    vec![
                        cformat!("* There are <y>{}</> modified files", msg.len()),
                        cformat!("  You have to commit or stash them"),
                    ]
                )
            }
            _ => {
                Error::new(
                    ErrorType::Unknown,
                    vec![
                        cformat!("<y>* Repository: <m,i>({})</>", path_str).as_str(),
                        cformat!("  <g>» Class: <m,i>({:?})</>", class_).as_str(),
                        cformat!("  <g>» Code : <m,i>({:?})</>", code).as_str(),
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

