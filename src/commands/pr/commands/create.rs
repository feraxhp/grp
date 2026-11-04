use std::{env::current_dir, fmt::Display};

use clap::{ArgMatches, Command, arg, command};
use color_print::cformat;
use grp_core::{ErrorType, Platform, animation::Animation, structs::{BranchIdentifier, PullRequestOptions}};
use ink_md::platform;
use mynox::{Context, Error, Extend, Formater, error};
use reqwest::Url;

use crate::{animations::animation::{self, Process}, commands::{config::config, core::{args::Arguments, commands::Commands}, validations::{branch::BranchStructure, or_exit::structure::OrExit, repo::RepoStructure}}, errors::ToError, local::{git::{options::Options, structs::GitUtils}, structs::{Git2Context, LocalError}}, printables::show::pconf, system::directories::BasicDir, usettings::structs::Usettings};

pub fn command() -> Command {
        Commands::create(cformat!("Create a <y,i>pull-request</> for the given platform"))
        .args([
            Arguments::branch_structure(false)
                .id("target")
                .help("The target repository to merge into")
            ,
            Arguments::branch_structure(false)
                .id("origin")
                .help("The origin repository to merge from (defaults to the current repo$branch)")
                .long("origin")
                .short('s')
                .conflicts_with("push")
            ,
            arg!(-p --push "If present, pushes the changes before creating the pr (upstream must be set)"),
            arg!(-d --draft "If present, sets the pull request as a draft "),
            arg!(-m --maintainer_can_modify "If present, allows maintainers to update your branch"),
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Preparing pull request");
    
    let maintainer_can_modify = args.get_flag("maintainer_can_modify");
    let draft = args.get_flag("draft");
    let push = args.get_flag("push");
    
    let (target, origin) = match (args.get_one::<BranchStructure>("target"), args.get_one::<BranchStructure>("origin")) {
        (None, None) => {
            animation.change_message("Autodetecting target and origin repos");
            let current = as_mynox(detect_current_repo(&usettings, &animation), &usettings).or_exit(&animation);
            (current.clone(), current)
        },
        (None, Some(o)) => {
            animation.change_message("Autodetecting target repo");
            let current = as_mynox(detect_current_repo(&usettings, &animation), &usettings).or_exit(&animation);
            (current, o.clone())
        },
        (Some(t), None) => {
            animation.change_message("Autodetecting origin repo");
            let current = as_mynox(detect_current_repo(&usettings, &animation), &usettings).or_exit(&animation);
            (t.clone(), current)
        },
        (Some(t), Some(o)) => (t.clone(), o.clone()),
    };
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: target.repo.owner,
            repo: target.repo.path,
            branch: target.branch,
        },
        origin: BranchIdentifier { 
            owner: origin.repo.owner, 
            repo: origin.repo.path, 
            branch: origin.branch, 
        },
        title: "help".to_string(),
        body: "This is suppsed to be a pr".to_string(),
        draft: false,
        maintainer_can_modify: false,
    };
    
    let pconf = target.repo.pconf.unwrap();
    let pconf = usettings.get_pconf_by_name(&pconf, &animation).or_exit(&animation);
    let config = pconf.to_config();
    
    let platform = Platform::matches(&pconf.r#type).or_exit(&animation);
    
    let pr = platform.create_pr(&options, &config, &animation).await.or_exit(&animation);
    
    dbg!(pr);
}

fn as_mynox<T>(result: Result<T, LocalError>, usettings: &Usettings) -> Result<T, mynox::Error> {
    result.map_err(|e| {
        match e {
            LocalError::Core(error) => error,
            LocalError::Git(error) => {
                let repo = match BasicDir::current() {
                    Ok(r) => r,
                    Err(e) => return e,
                };
                
                let context = Git2Context {
                    action: crate::local::git::structs::Action::Push,
                    owner: "",
                    repo: &repo.to_string_lossy(),
                    config: None,
                    usettings,
                };
                
                error.to_error(context)
            },
        }
    })
}

fn detect_current_repo<A: Animation + ?Sized>(usettings: &Usettings, animation: &Box<A>) -> Result<BranchStructure, LocalError> {
    let repo = git2::Repository::discover(".")?;
    let branch_name = GitUtils::get_branch_name(&repo)?;
    
    if branch_name.is_empty() {
        return Err(LocalError::Git(git2::Error::new(
            git2::ErrorCode::UnbornBranch, git2::ErrorClass::Callback,
            "Impossible to auto-detect branch",
        )));
    }
    
    let branch = repo.find_branch(&branch_name, git2::BranchType::Local)?;
    let remote = GitUtils::get_remote_from_branch(&repo, &branch)?;
    
    let pconf = usettings.get_raw_pconf_by_name(&remote).or_exit(animation);
    
    let remote = repo.find_remote(&remote)?;
    let url = remote.url()?;
    
    let url = match Url::parse(url) {
        Ok(u) => u,
        Err(e) => {
            return Err(
                LocalError::Core(
                    error_autodetect(url, pconf.name)
                        .extend_detail(e.to_string())
                )
            )
        },
    };

    let full_path = match url.path_segments() {
        Some(ps) => ps.collect::<Vec<_>>(),
        None => return Err(
            LocalError::Core(
                error_autodetect(url, pconf.name)
                    .extend_note("The url should have a path to be recongnized".as_tip())
            )
        ),
    };
    
    if full_path.len() < 2 { // the path should at least be 2 deep
        return Err(LocalError::Core(error_autodetect(url, pconf.name)));
    }
    
    let last = full_path.last().unwrap();
    let path = last.replace(".git", "");
    
    let owner = full_path.get(..(full_path.len() -1)).unwrap().join("/");
    
    Ok(BranchStructure { 
        repo: RepoStructure {
            pconf: Some(pconf.name),
            owner, path,
            len: full_path.len() - 1,
        }, 
        branch: branch_name
    })
}

fn error_autodetect(url: impl Display, remote: impl Display) -> Error {
    error!(
        "auto::detect::current::repo",
        "Imposible to autodetect the current repo structure",
        "The url of the remote is mal formed",
        => format!("» **url**: _<raw>{}</raw>_", url),
           format!("» **remote**: _<raw>{}</raw>_", remote)
    )
}