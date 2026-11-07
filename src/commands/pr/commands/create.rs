
use std::io;
use std::process::exit;

use clap::{ArgMatches, Command, arg};
use color_print::ceprint;
use color_print::cprintln;
use color_print::cformat;

use grp_core::structs::PullRequestOptions;
use grp_core::structs::BranchIdentifier;
use grp_core::animation::Animation;
use grp_core::Platform;
use mynox::Formater;
use mynox::error;

use crate::commands::validations::or_exit::structure::OrExit;
use crate::commands::validations::branch::BranchStructure;
use crate::commands::validations::repo::RepoStructure;
use crate::commands::validations::DoubleOption;
use crate::commands::core::commands::Commands;
use crate::commands::core::args::Arguments;
use crate::animations::animation::Process;
use crate::animations::animation::Suspend;
use crate::commands::validations::PconfOr;
use crate::system::directories::BasicDir;
use crate::usettings::structs::Usettings;
use crate::local::git::utils::GitUtils;
use crate::local::structs::Git2Context;
use crate::local::structs::LocalError;
use crate::local::structs::Local;
use crate::system::input::Input;
use crate::errors::ToError;
use crate::success;

pub fn command() -> Command {
        Commands::create(cformat!("Create a <y,i>pull-request</> for the given platform"))
        .args([
            Arguments::pconf_or::<BranchStructure>("target", "repo?branch", false)
                .help("The target repository to merge into")
            ,
            arg!([title] "The title of the pull request (if not present, is ask interactively)"),
            // arg!([body] "The title of the pull request (if not present, is ask interactively)"),
            Arguments::branch_structure(false)
                .id("origin")
                .help("The origin repository to merge from (defaults to the current repo$branch)")
                .long("origin")
                .short('s')
            ,
            arg!(-d --draft "If present, sets the pull request as a draft "),
            arg!(-m --maintainer_can_modify "If present, allows maintainers to update your branch"),
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Process::new("Preparing pull request");
    
    let title = args.get_one::<String>("title").cloned();
    // let body = args.get_one::<String>("body").cloned();
    let maintainer_can_modify = args.get_flag("maintainer_can_modify");
    let draft = args.get_flag("draft");
    
    let target = args.get_one::<PconfOr<BranchStructure>>("target").unwrap_or_default();
    
    let (pconf, target) = match target {
        DoubleOption::None => (None, None),
        DoubleOption::First(p) => (Some(p), None),
        DoubleOption::Second(t) => (None, Some(t)),
    };
    
    let raw_pconf = pconf.map(|p| p.as_raw());
    let config = pconf.map(|p| p.to_config());
    
    let (target, origin, repo) = match (target, args.get_one::<BranchStructure>("origin")) {
        (None, None) => {
            animation.change_message("Autodetecting target and origin repos");
            let repo = GitUtils::discover(".").or_exit(&animation);
            
            let info = as_mynox(repo.autodetect(raw_pconf), &usettings).or_exit(&animation);
            let default = repo.get_default_branch(&info.remote, config.as_ref()).map_err(Into::into);
            let default = as_mynox(default, &usettings).or_exit(&animation);
            
            let repo_structure = RepoStructure {
                pconf: Some(info.remote),
                owner: info.owner,
                path: info.repo,
                len: info.depth,
            };
            
            let target = BranchStructure { repo: repo_structure.clone(), branch: default };
            let current = BranchStructure { repo: repo_structure, branch: info.branch };
            (target, current, Some(repo))
        },
        (None, Some(o)) => {
            animation.change_message("Autodetecting target repo");
            let repo = GitUtils::discover(".").or_exit(&animation);
            
            let info = as_mynox(repo.autodetect(raw_pconf), &usettings).or_exit(&animation);
            let default = repo.get_default_branch(&info.remote, config.as_ref()).map_err(Into::into);
            let default = as_mynox(default, &usettings).or_exit(&animation);
            
            let repo_structure = RepoStructure {
                pconf: Some(info.remote),
                owner: info.owner,
                path: info.repo,
                len: info.depth,
            };
            
            let target = BranchStructure { repo: repo_structure, branch: default };
            (target, o.clone(), Some(repo))
        },
        (Some(t), None) => {
            animation.change_message("Autodetecting origin repo");
            let repo = GitUtils::discover(".").or_exit(&animation);
            
            let info = as_mynox(repo.autodetect(raw_pconf), &usettings).or_exit(&animation);
            
            let repo_structure = RepoStructure {
                pconf: Some(info.remote),
                owner: info.owner,
                path: info.repo,
                len: info.depth,
            };
            
            let origin = BranchStructure { repo: repo_structure, branch: info.branch };
            (t.clone(), origin, Some(repo))
        },
        (Some(t), Some(o)) => (t.clone(), o.clone(), None),
    };
    
    let tr = (&target).to_string();
    let or = (&origin).to_string();
    if tr == or {
        Err::<(), _>(
            error!(
                "invalid::pr::same::branch", "Invalid target for PR",
                "The _target_ and _origin_ cant be the same",
                => format!("» **target**: +_<raw>{tr}</raw>_+"),
                   format!("» **origin**: -_<raw>{or}</raw>_-"),
            )
        ).or_exit(&animation)
    }
    
    animation.change_message("Loading repository pr template");
    
    let pconf = match pconf {
        Some(p) => p.clone(),
        None => usettings.get_pconf_or_default(&target.repo.pconf.unwrap_or_default(), &animation).or_exit(&animation),
    };
    
    let platform: Local = Local(Platform::matches(&pconf.r#type).or_exit(&animation));
    
    let templates = match &repo {
        Some(repo) => platform.load_pr_templates(repo).or_exit(&animation),
        None => {
            animation.spinners.get(0).unwrap()
                .println(cformat!("<y>Downloading templates from <i>remote repos</i>, is not yet suported</>"))
            ;
            Vec::new()
        },
    };
    
    let title = match title {
        Some(t) => t,
        None => {
            animation.suspend(|| {
                Input::text("Title", None)
            }).or_exit(&animation)
        },
    };
    
    let template = match templates.len() > 1 {
        true => {
            animation.suspend(|| {
                Input::raw_select("Template", None, templates)
            })
            .or_exit(&animation)
            .content
        },
        false => {
            templates.get(0).map(|t| t.content.to_owned()).unwrap_or_default()
        },
    };

    let body = Input::use_editor(
            &template, 
            "PR_TEMPLATE.md", 
            &usettings, 
            repo.as_ref(), 
            &animation
        )
        .or_exit(&animation);
    
    let body_trimmed = body.trim();
    let template_trimmed = template.trim();
    let has_template = !template_trimmed.is_empty();
    
    let body = match has_template {
        true if body_trimmed == template_trimmed 
        => Err(error!(
            "input::unchanged::template", "Unchanged body",
            "Yout did not add any text to the _`template`_",
            "It is expected that you _+set a description+_" =>
            "Please fill in the template before submitting the *PR*".as_tip()
        )),
        true if body_trimmed.is_empty() 
        => Err(error!(
            "input::empty::body", "Aborting PR creation",
            "You cleared the _template_ there for the _*PR* is -canceled-_",
        )),
        false if body_trimmed.is_empty() 
        => {
            let confirmation = animation.suspend(|| {
                ceprint!( "<y>Create <c>PR</c> with an <dim><<empty>></dim> body [y/<r>N</r>]?: </>");
                
                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)
                        .expect("Something went wrong while reading the input");
                    
                    let input = input.trim();
                    if input.is_empty() { break false; }
                    if input.to_lowercase() == "n" { break false; }
                    if input.to_lowercase() == "y" { break true; }
                    
                    ceprint!("<y>Type <r>'y'</r> to confirm or <g>'n'</g> to <dim,i>cancel</dim,i>: </>");
                }
            });
            
            match confirmation {
                true => Ok(String::new()),
                false => {
                    animation.finish_with_success(success!(np "pr creation" "canceled!"));
                    exit(0);
                },
            }
        },
        true | false => Ok(body_trimmed.to_string()),
    }.or_exit(&animation);
    
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
        title: title,
        body: body,
        draft,
        maintainer_can_modify,
    };
    let config = pconf.to_config();
    let pr = platform.create_pr(&options, &config, &animation).await.or_exit(&animation);
    
    animation.finish_with_success(success!(np "create pr"));
    cprintln!("see <i>pull-request</i> at: <b,i,u>{}</>", pr.url);
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
                    owner: None,
                    repo: &repo.to_string_lossy(),
                    config: None,
                    usettings: Some(usettings),
                };
                
                error.to_error(context)
            },
        }
    })
}
