use color_print::cformat;
use git2::ErrorCode;
use crate::errors::error::Error;
use crate::errors::types::ErrorType;

impl Error {
    pub(crate) fn git_to_local(error: git2::Error, repo: String, pconf: String) -> Error {
        let code = error.code();

        match code {
            ErrorCode::NotFound => {
                Error::new(
                    ErrorType::NotFoundRepo,
                    vec![
                        repo.as_str(),
                        cformat!("pconf: \"<g,i>{}</>\"", pconf).as_str()
                    ]
                )
            }
            ErrorCode::Auth => {
                Error::new(
                    ErrorType::BadTokenScope,
                    vec![
                        pconf.as_str(),
                        "read/write repo",
                    ]
                )
            }
            ErrorCode::Exists => {
                let mut content: Vec<String> = Vec::new();
                if error.message().contains(" exists and is not an empty directory"){
                    let path = error.message().replace("'","");
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
            ErrorCode::GenericError if error.message().contains("unexpected http status code: 404") => {
                Error::new(
                    ErrorType::NotFoundRepo,
                    vec![
                        repo.as_str(),
                    ]
                )
            }
            _ => {
                Error::new(
                    ErrorType::Unknown,
                    vec![
                        cformat!("Repository: <m,i>({})</>", repo).as_str(),
                        error.message(),
                    ]
                )
            }
            // ErrorCode::Ambiguous |
            // ErrorCode::BufSize |
            // ErrorCode::User |
            // ErrorCode::BareRepo |
            // ErrorCode::UnbornBranch |
            // ErrorCode::Unmerged |
            // ErrorCode::NotFastForward |
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

