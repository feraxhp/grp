use color_print::cformat;
use git2::{ErrorClass, ErrorCode};

use crate::girep::config::Config;
use crate::girep::error::structs::Error;
use crate::girep::error::types::ErrorType;
use crate::local::git::structs::Action;


impl Error {
    #[allow(unused_variables)]
    pub fn from_git2<T: AsRef<str>>(ge: git2::Error, action: Action, repo: &T, config: Option<&Config>) -> Self {
        let code = ge.code();
        let class_ = ge.class();
        let message = ge.message().to_string();
        let repo = repo.as_ref();
        
        match (code, class_, message.as_str(), action) {
            (ErrorCode::NotFound, ErrorClass::Repository, _, _) => {
                Error::new(ErrorType::LocalRepoNotFound, vec![repo])
            }
            (ErrorCode::NotFound, ErrorClass::Config, msg, action) if msg.starts_with("-u ") => {
                Error::new_custom(
                    "No upstream set".to_string(),
                    vec![
                        cformat!("<y>* The current branch has no <i>Upstream</> set"),
                        cformat!("  You can set it by running the command:"),
                        cformat!("  •<g> grp {} {}</>", action.as_str(), msg),
                    ]
                )
            }
            (ErrorCode::Exists, ErrorClass::Config, message,Action::SetRemote(name, url)) => {
                let command = cformat!("    <g>•</g> <i>git remote set-url {} {}</>", &name, &url);
                Error::new(
                    ErrorType::AlreadyExists, 
                    vec![
                        "Remote".to_string(), name, 
                        cformat!("  <y>» You may whant to run</>"), 
                        command
                    ]
                )
            }
            (ErrorCode::GenericError, ErrorClass::Http, message, Action::Clone(platform)) if message == "request failed with status code: 404" => {
                Error::new(ErrorType::NotRepoFound, vec!["", repo, &platform])
            }
            (ErrorCode::NotFound, ErrorClass::Config, "no pconf", _) if message == "request failed with status code: 404" => {
                Error::new(
                    ErrorType::Obj404,
                    vec![
                        "Default Pconf",
                        "No default Pconf configured",
                    ]
                )
            }
            (ErrorCode::GenericError, ErrorClass::Http, "too many redirects or authentication replays", _) => {
                Error::new(
                    ErrorType::Unauthorized, 
                    vec![
                        match config {
                            Some(c) => c.pconf.clone(),
                            None => "default".to_string()
                        },
                        "oAuth".to_string()
                    ]
                )
            }
            (ErrorCode::NotFastForward, ErrorClass::Reference, "cannot push non-fastforwardable reference", _) => {
                Error::new_custom(
                    "No fast-forward push".to_string(),
                    vec![
                        cformat!("<y>* The branch conflicts with the remote"),
                        cformat!("  you have to solved it fist"),
                        cformat!("  or <i,m>add the <r>--force</r> tag</>"),
                    ]
                )
            }
            (code, class_, message,action) => {
                Error::new_custom(
                    message.to_string(),
                    vec![
                        cformat!("<y>* Repository: <m,i>({})</>", repo),
                        cformat!("  <g>» Code   : <m,i>({:?})</>", code),
                        cformat!("  <g>» Class  : <m,i>({:?})</>", class_),
                        cformat!("  <g>» Action : <m,i>({})</>", action.as_str()),
                    ]
                )
            }
        }
    }
}
