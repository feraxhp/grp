use git2::ErrorClass;
use git2::ErrorCode;

use mynox::Formater;
use mynox::Error;
use mynox::error;

use crate::local::structs::Git2Context;
use crate::errors::ToError;

use super::super::git::structs::Action;

macro_rules! etype {
    ($lit:literal) => { concat!("git::", $lit) };
}

impl ToError for git2::Error {
    type Context<'a> = Git2Context<'a>;
    
    fn to_error<'a>(self, context: Self::Context<'a>) -> Error {
        let action = context.action;
        let owner = context.owner;
        let repo = context.repo;
        let config = context.config;
        let usettings = context.usettings;
        
        let code = self.code();
        let class_ = self.class();
        let message = self.message();
        
        match (code, class_, message, action) {
            (ErrorCode::NotFound, ErrorClass::Repository, _, _) => {
                error!{
                    etype!("not_found::repository::local"), 
                    "The repository can not be found", 
                    "Is this directory a git repository?",
                    format!("» **repo**: _{repo}_"),
                    => 
                    "To create a new repo you can run".as_tip(),
                    "git init".as_command(),
                }
            }
            (ErrorCode::NotFound, ErrorClass::Config, msg, action) if msg.starts_with("-u ") => {
                error!{
                    etype!("not_found::upstream"),
                    "No upstream set",
                    format!("The current branch has no _upstream_ set"),
                    =>
                    "You can set it by running the command:".as_tip(),
                    format!("grp {} {}", action, msg).as_command(),
                }
            }
            (ErrorCode::Exists, ErrorClass::Invalid, msg,Action::Clone) if msg.ends_with("exists and is not an empty directory") => {
                error!{
                    etype!("already_exist::directory"),
                    "Error cloning the remote",
                    format!("The directory already exist: +{repo}+"),
                    =>
                    "You may want to delete or especify a difrent directory.".as_tip(),
                }
            }
            (ErrorCode::Exists, ErrorClass::Config, _,Action::SetRemote(name, url)) => {
                error!{
                    etype!("already_exist::remote"),
                    "Error setting a new remote",
                    format!("The remote already exist"),
                    =>
                    "You may want to run the command:".as_tip(),
                    format!("git remote set-url {} {}", name, url).as_command(),
                }
            }
            (ErrorCode::GenericError, ErrorClass::Http, _, Action::Clone) => {
                let pconf = config.map_or_else(|| { &usettings.default }, |e| &e.pconf);
                
                error!{
                    etype!("not_found::repository::remote"),
                    "Error cloning the repository",
                    format!("The repository does not exist"),
                    =>
                    if !repo.to_string().contains("[") { 
                        format!("**<<{}:{}/{}>>**", pconf, owner, repo) 
                    }
                    else { 
                        format!("**{}**", repo) 
                    },
                }
            }
            (ErrorCode::GenericError, ErrorClass::Os, message, _) if message == "failed to send request: The server name or address could not be resolved" => {
                error!{
                    etype!("request::error"),
                    "Error getting the configuration",
                    "The server name or address could not be resolved",
                    =>
                    "Please check your internet connection".as_tip(),
                }
            }
            (ErrorCode::NotFound, ErrorClass::Config, "no pconf", _) if message == "request failed with status code: 404" => {
                error!{
                    etype!("not_found::config::default::pconf"),
                    "Error getting the configuration",
                    format!("There is no default pconf configured"),
                }
            }
            (ErrorCode::GenericError, ErrorClass::Http, "too many redirects or authentication replays", _) => {
                let pconf = config.map_or_else(|| { &usettings.default }, |e| &e.pconf);
                
                error!{
                    etype!("unauthorized"),
                    "too many redirects or authentication replays",
                    "Please check your token and the repo access",
                    =>
                    format!("pconf: **{pconf}**"),
                    format!("owner: **{owner}**"),
                }
            }
            (ErrorCode::NotFastForward, ErrorClass::Reference, _, Action::Push) => {
                error!{
                    etype!("conflict::remote"),
                    "No fast-forward push",
                    "The branch conflicts with the remote",
                    =>
                    format!("you have to solved it fist"),
                    format!("or _`add the- ----force- tag`_"),
                }
            }
            (ErrorCode::Conflict, ErrorClass::Merge, _, Action::Pull) => {
                error!{
                    etype!("conflict::local"),
                    "No fast-forward pull",
                    "The remote branch conflicts with local changes",
                    =>
                    format!("you have to solved it fist"),
                    format!("or _`add the- ----force- tag`_"),
                }
            }
            (ErrorCode::Unmerged, ErrorClass::Rebase, "unstaged changes exist in workdir", Action::Pull) => {
                error!{
                    etype!("conflict::rebase"),
                    "Rebase operation conflict",
                    "The remote branch conflicts with local changes",
                    =>
                    format!("you can finish the rebase as you").as_tip(),
                    format!("normaly does").as_tip_cotinuation(),
                    String::new(),
                    "You can abort it by runing the command:".as_tip(),
                    "git rebase --abort".as_command(),
                }
            }
            (ErrorCode::Locked, ErrorClass::Merge, m, Action::Pull) => {
                let files = m.split(",")
                    .map(|f| format!("  +→+ _`{}`_", f.trim()))
                    .collect::<Vec<_>>();
                
                let mut messages = Vec::with_capacity(files.len());
                messages.extend(files);
                
                Error::new(
                    etype!("conflict::merge"),
                    "No fast-forward merge",
                    "The merge operation was locked by:",
                    messages,
                    [
                        "Please _`commit`_ or _`stash`_ them",
                        "or _`add the- ----force- tag`_ to override them",
                    ],
                )
            }
            (ErrorCode::NotFound, ErrorClass::Config, m, _) => {
                error!{
                    etype!("not_found::config"),
                    "Git interaction error", m,
                }
            }
            (ErrorCode::NotFound, ErrorClass::Reference, m, _) => { 
                error!{
                    etype!("not_found::reference"),
                    "Git interaction error", m,
                }
            }
            (ErrorCode::NotFound, ErrorClass::Merge, m, Action::Pull) if m.contains("r:") => {
                error!{
                    etype!("not_found::branch::local"),
                    "No base found to merge",
                    "This happens when the _`local`_ and _`remote`_",
                    "banches don't share a common ancestor",
                    =>
                    "Manually resolve this by *cloning the remote repository*".as_tip(),
                    "and then *copying your +local changes+* into it.".as_tip_cotinuation(),
                }
            }
            (code, class_, message,action) => {
                error!{
                    etype!("unmapped"),
                    "Unmaped error happends",
                    message,
                    format!("repo: *{}*", repo).as_tip(),
                    format!("code: _+{:?}+_", code).as_tip(),
                    format!("clas: _`{:?}`_", class_).as_tip(),
                    format!("actn: _+{}+_", action).as_tip(),
                }
            }
        }
    }
}
