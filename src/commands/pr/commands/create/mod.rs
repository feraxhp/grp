mod detect;


use clap::{ArgMatches, Command, arg};
use color_print::cformat;

use grp_core::structs::PullRequestOptions;
use grp_core::structs::BranchIdentifier;
use grp_core::animation::Animation;
use grp_core::Platform;

use crate::commands::pr::commands::create::detect::detect_current_repo;
use crate::{animations::animation::Process, commands::{core::{args::Arguments, commands::Commands}, validations::{branch::BranchStructure, or_exit::structure::OrExit}}, errors::ToError, local::structs::{Git2Context, LocalError}, system::directories::BasicDir, usettings::structs::Usettings};

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
            let (current, default) = as_mynox(detect_current_repo(&usettings, &animation, true), &usettings).or_exit(&animation);
            
            let mut target = current.clone();
            target.branch = default;
            
            (target, current)
        },
        (None, Some(o)) => {
            animation.change_message("Autodetecting target repo");
            let (mut current, default) = as_mynox(detect_current_repo(&usettings, &animation, true), &usettings).or_exit(&animation);
            current.branch = default;
            
            (current, o.clone())
        },
        (Some(t), None) => {
            animation.change_message("Autodetecting origin repo");
            let (current, _) = as_mynox(detect_current_repo(&usettings, &animation, false), &usettings).or_exit(&animation);
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
