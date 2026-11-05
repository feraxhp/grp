mod detect;

use clap::{ArgMatches, Command, arg};
use color_print::cformat;

use grp_core::structs::PullRequestOptions;
use grp_core::structs::BranchIdentifier;
use grp_core::animation::Animation;
use grp_core::Platform;
// use mynox::error;

use crate::commands::pr::commands::create::detect::detect_current_repo;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::commands::validations::branch::BranchStructure;
use crate::commands::core::commands::Commands;
use crate::commands::core::args::Arguments;
use crate::animations::animation::Process;
use crate::system::directories::BasicDir;
use crate::usettings::structs::Usettings;
use crate::local::structs::Git2Context;
use crate::local::structs::LocalError;
use crate::local::structs::Local;
use crate::system::input::Input;
use crate::errors::ToError;

pub fn command() -> Command {
        Commands::create(cformat!("Create a <y,i>pull-request</> for the given platform"))
        .args([
            Arguments::branch_structure(false)
                .id("target")
                .help("The target repository to merge into")
            ,
            arg!([title] "The title of the pull request (if not present, is ask interactively)"),
            // arg!([body] "The title of the pull request (if not present, is ask interactively)"),
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

#[allow(unused)]
pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Preparing pull request");
    
    let title = args.get_one::<String>("title").cloned();
    // let body = args.get_one::<String>("body").cloned();
    let maintainer_can_modify = args.get_flag("maintainer_can_modify");
    let draft = args.get_flag("draft");
    let push = args.get_flag("push");
    
    let (target, origin, repo) = match (args.get_one::<BranchStructure>("target"), args.get_one::<BranchStructure>("origin")) {
        (None, None) => {
            animation.change_message("Autodetecting target and origin repos");
            let (current, default, repo) = as_mynox(detect_current_repo(&usettings, &animation, true), &usettings).or_exit(&animation);
            
            let mut target = current.clone();
            target.branch = default;
            
            (target, current, Some(repo))
        },
        (None, Some(o)) => {
            animation.change_message("Autodetecting target repo");
            let (mut current, default, repo) = as_mynox(detect_current_repo(&usettings, &animation, true), &usettings).or_exit(&animation);
            current.branch = default;
            
            (current, o.clone(), Some(repo))
        },
        (Some(t), None) => {
            animation.change_message("Autodetecting origin repo");
            let (current, _, repo) = as_mynox(detect_current_repo(&usettings, &animation, false), &usettings).or_exit(&animation);
            (t.clone(), current, Some(repo))
        },
        (Some(t), Some(o)) => (t.clone(), o.clone(), None),
    };
    
    animation.change_message("Loading repository pr template");
    let pconf = target.repo.pconf.unwrap();
    let pconf = usettings.get_pconf_by_name(&pconf, &animation).or_exit(&animation);
    
    let platform: Local = Local(Platform::matches(&pconf.r#type).or_exit(&animation));
    
    let templates = match &repo {
        Some(repo) => platform.load_pr_templates(repo).or_exit(&animation),
        None => todo!(),
    };
    
    let body = match templates.len() > 1 {
        true => todo!(),
        false => {
            let base_text = templates.get(0).map(|t| t.content.as_str()).unwrap_or_default();
            let body = Input::use_editor(
                    base_text, 
                    "PR_TEMPLATE.md", 
                    &usettings, 
                    repo.as_ref(), 
                    &animation
                )
                .or_exit(&animation);
            
            // if body.as_str() == base_text {
            //     Err::<(), mynox::Error>(error!(
            //         "input::unchanged::template", "Unchange body", 
            //         "Yout did not add any text to the _`template`_",
            //         "It is espected that you _+add a description+_,",
            //         "before sending the _pr_"
            //     )).or_exit(&animation);
            // }
            
            body
        },
    };
    
    animation.change_message("Finishing data recopilation");
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
        body: body,
        draft: false,
        maintainer_can_modify: false,
    };
    let config = pconf.to_config();
    dbg!(options);
    unreachable!();
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
