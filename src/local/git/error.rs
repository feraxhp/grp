use std::fmt::Display;

use color_print::cformat;
use git2::{ErrorClass, ErrorCode};

use grp_core::Error;
use grp_core::Config;
use grp_core::Formater;
use grp_core::empty_notes;
use grp_core::errors::Request;
use crate::usettings::structs::Usettings;

use super::super::git::structs::Action;
use super::super::structs::Git2Error;

macro_rules! etype {
    ($lit:literal) => { concat!("git::", $lit) };
}

macro_rules! make_error {
    (
        $type:expr,
        $message:expr,
        $detail:expr,
        $( $explain:expr, )* $(,)?
    ) => {
        Error::new(
            etype!($type), $message, $detail, 
            vec![
                $( $explain, ) *
            ],
            empty_notes!()
        )
    };
}

impl Git2Error for Error {
    fn from_git2<
        T: Display,
        S: Display,
    >(
        ge: git2::Error, 
        action: Action, 
        owner: S,
        repo: T, 
        config: Option<&Config>,
        usettings: &Usettings,
    ) -> Error {
        let code = ge.code();
        let class_ = ge.class();
        let message = ge.message();
        
        match (code, class_, message, action) {
            (ErrorCode::NotFound, ErrorClass::Repository, _, _) => {
                make_error!{
                    "not_found::repository::local", 
                    "The repository can not be found", 
                    "Is this directory a git repository?",
                    cformat!("  <b><<{}>></>", repo),
                    format!(""),
                    "To create a new repo you can run".as_tip(),
                    "git init".as_command(),
                }
            }
            (ErrorCode::NotFound, ErrorClass::Config, msg, action) if msg.starts_with("-u ") => {
                make_error!{
                    "not_found::upstream",
                    "No upstream set",
                    cformat!("The current branch has no <i>Upstream</> set"),
                    "You can set it by running the command:".as_tip(),
                    format!("grp {} {}", action, msg).as_command(),
                }
            }
            (ErrorCode::Exists, ErrorClass::Config, _,Action::SetRemote(name, url)) => {
                make_error!{
                    "already_exist::remote",
                    "Error setting a new remote",
                    cformat!("The remote already exist"),
                    "You may whant to run the command:".as_tip(),
                    format!("git remote set-url {} {}", &name, &url).as_command(),
                }
            }
            (ErrorCode::GenericError, ErrorClass::Http, _, Action::Clone) => {
                let pconf = config.map_or_else(|| { &usettings.default }, |e| &e.pconf);
                
                make_error!{
                    "not_found::repository::remote",
                    "Error cloning the repository",
                    cformat!("The repository does not exist"),
                    if !repo.to_string().contains("[") { 
                        cformat!("  <b><<{}:{}/{}>></>", pconf, owner, repo) 
                    }
                    else { 
                        cformat!("  <b>{}</>", repo) 
                    },
                }
            }
            (ErrorCode::GenericError, ErrorClass::Os, message, _) if message == "failed to send request: The server name or address could not be resolved" => {
                Request::fetch("Please check your internet connection", empty_notes!())
            }
            (ErrorCode::NotFound, ErrorClass::Config, "no pconf", _) if message == "request failed with status code: 404" => {
                make_error!{
                    "not_found::config::default::pconf",
                    "Error getting the configuration",
                    cformat!("There is no default pconf configured"),
                }
            }
            (ErrorCode::GenericError, ErrorClass::Http, "too many redirects or authentication replays", _) => {
                let pconf = config.map_or_else(|| { &usettings.default }, |e| &e.pconf);
                
                Request::unauthorized(pconf, owner, empty_notes!())
            }
            (ErrorCode::NotFastForward, ErrorClass::Reference, _, Action::Push) => {
                make_error!{
                    "conflict::remote",
                    "No fast-forward push",
                    "The branch conflicts with the remote",
                    cformat!("  you have to solved it fist"),
                    cformat!("  or <i,m>add the <r>--force</r> tag</>"),
                }
            }
            (ErrorCode::Conflict, ErrorClass::Merge, _, Action::Pull) => {
                make_error!{
                    "conflict::local",
                    "No fast-forward pull",
                    "The remote branch conflicts with local changes",
                    cformat!("  you have to solved it fist"),
                    cformat!("  or <i,m>add the <r>--force</r> tag</>"),
                }
            }
            (ErrorCode::Unmerged, ErrorClass::Rebase, "unstaged changes exist in workdir", Action::Pull) => {
                make_error!{
                    "conflict::rebase",
                    "Rebase operation conflict",
                    "The remote branch conflicts with local changes",
                    cformat!("  you can finish the rebase as you"),
                    cformat!("  normaly does"),
                    cformat!(""),
                    "You can abort it by runing the command:".as_tip(),
                    "git rebase --abort".as_command(),
                }
            }
            (ErrorCode::Locked, ErrorClass::Merge, m, Action::Pull) => {
                let files = m.split(",")
                    .map(|f| cformat!("  <g>→</g> <m,i>{}</>", f.trim()))
                    .collect::<Vec<_>>();
                
                let mut messages = Vec::with_capacity(files.len() + 2);
                messages.extend(files);
                messages.push(cformat!("<y>* Please <m,i>commit</m,i> or <m,i>stash</m,i> them</>"));
                messages.push(cformat!("<y>  or <m,i>add the <r>--force</r> tag</m,i> to override them</>"));
                
                Error::new(
                    etype!("conflict::merge"),
                    "No fast-forward merge",
                    "The merge operation was locked by:",
                    messages,
                    empty_notes!()
                )
            }
            (ErrorCode::NotFound, ErrorClass::Config, m, _) => {
                make_error!{
                    "not_found::config",
                    "Git interaction error",
                    m,
                }
            }
            (ErrorCode::NotFound, ErrorClass::Reference, m, _) => { 
                make_error!{
                    "not_found::reference",
                    "Git interaction error",
                    m,
                }
            }
            (ErrorCode::NotFound, ErrorClass::Merge, m, Action::Pull) if m.contains("r:") => {
                make_error!{
                    "not_found::branch::local",
                    "No base found to merge",
                    "This happens when the <m,i>local</m,i> and <m,i>remote",
                    cformat!("<y>  banches don't share a common ancestor.</>"),
                    cformat!(""),
                    "Manually resolve this by cloning the remote repository".as_tip(),
                    "and then copying your local changes into it.".as_tip_cotinuation(),
                }
            }
            (code, class_, message,action) => {
                make_error!{
                    "unmapped",
                    "Unmaped error happends",
                    message,
                    cformat!("repo: <c>{}</>", repo).as_tip(),
                    cformat!("code: <g,i>{:?}</>", code).as_tip(),
                    cformat!("clas: <m,i>{:?}</>", class_).as_tip(),
                    cformat!("actn: <g,i>{}</>", action).as_tip(),
                }
            }
        }
    }
}
