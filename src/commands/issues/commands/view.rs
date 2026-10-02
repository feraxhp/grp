use std::process::exit;
use futures::StreamExt;

use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};
use grp_core::animation::Animation;
use grp_core::structs::{Comment, Issue};
use grp_core::{Error, Formater, Platform};

use crate::animations::animation::Fetch;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::issues::IssueStructure;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::printables::humanize::Humanize;
use crate::printables::markdown::Markdown;
use crate::printables::show::Show;
use crate::printables::stylizer::Stylizer;
use crate::usettings::structs::Usettings;

pub fn command() -> Command {
    Commands::view("View comments for a specific issue")
        .args([
            Arguments::issue_structure(true),
            arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing issue fetch");

    let issue = args.get_one::<IssueStructure> ("issue").unwrap();
    let pconf = match issue.repo.pconf.clone() {
        Some(e) => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        None => usettings.get_default_pconf().or_exit(&animation),
    };
    
    let owner = Some(&issue.repo.owner);
    let show_errors = args.get_flag("show-errors");
    
    let platform = match Platform::matches(&pconf.r#type) {
        Ok(p) => p,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1)
        },
    };
    let config = pconf.to_config();

    animation.change_message("Getting issue details...");
    let issue_detail = match platform.issue(owner, &issue.repo.path, &issue.number, &config, &animation).await {
        Ok(s) => s,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            return;
        },
    };
    
    animation.change_message("Getting issue comments...");
    let stream = match platform.list_issue_comments(owner, &issue.repo.path, &issue.number, &config, &animation).await {
        Ok(s) => s,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            return;
        },
    };
    
    let an = &animation;
    let (comments, mut errors) = stream
        .enumerate()
        // .take(1)
        .map(|(i, s)| {
            match &s {
                Ok(pr) if let Some(last) = pr.pager.last_number() => {
                    an.change_message(cformat!("Getting issue comments: <i, u>page: {} of {}</>", i + 1, &last))
                },
                _ => an.change_message(cformat!("Getting issue comments: <i, u>page: {}</>", i + 1)),
            }
            s
        })
        .fold((vec![], vec![]), async move |curr, act| {
            let (mut repos, mut errors) = curr;
            match act {
                Ok(r) => repos.extend(r.result),
                Err(e) => errors.push(e),
            }
            (repos, errors)
        })
        .await;

    let (chunks, errors__) = print_issue(&issue_detail, comments);
    
    errors.extend(errors__);
    
    match (chunks.is_empty(), errors.is_empty()) {
        (true, true) => { unreachable!("This must be unrechable") },
        (false,  true) => {
            animation.finish_with_success(cformat!("<y,i>issues fetch</y,i> <g>succeeded!</>"));
            chunks.print_pretty();
        },
        (true, false) => {
            let error = Error::collection(errors);
            animation.finish_with_error(format!("{}", error.message));
            error.show();
        },
        (false, false) => {
            animation.finish_with_warning(cformat!("<m,i>{}</m,i> <y>finish with errors!</>", issue_detail.title));
            chunks.print_pretty();
            if show_errors { errors.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
    }
}

fn print_issue(issue: &Issue, comments: Vec<Comment>) -> (Vec<String>, Vec<Error>) {
    let mut chunks = Vec::with_capacity(6 + comments.len() * 5 + 1); // Improve latter on
    let mut errors = Vec::with_capacity(comments.len() + 1);

    macro_rules! lazy_parse {
        ($errors: expr, $text: expr, $default: expr, $prefix: literal) => {{
            let text = match $text.parse() {
                Ok(s) => s.trim_end_matches("\n").to_string(),
                Err(e) => {
                    $errors.push(e);
                    $text.to_owned().unwrap_or($default)
                },
            };
            
            text.lines()
                .filter_map(|s| {
                    let content = s.get(2..).unwrap_or("");
                    Some(cformat!("<b, bold>{} </><b>│</> {content}", $prefix))
                })
                .collect::<Vec<_>>()
                .join("\n")
        }};
    }
    
    chunks.push(cformat!("\n<b, bold> █ {}</> <i, dim>{}</>", &issue.title, cformat!("#{}", issue.number).as_link(&issue.url)));
    chunks.push(cformat!("<b, bold> █"));
    chunks.push(cformat!("<b, bold> █ </><b>╭──</> <green>{}</> <dim>- {}</>",issue.author.to_link(), issue.created_at.to_human()));
    chunks.push(lazy_parse!(errors, issue.body, cformat!("<dim, i>no detail</>"), " █"));
    chunks.push(cformat!("<b, bold> █ </><b>╰──</> ")); 
    
    let length = comments.len();
    for (index, comment) in (&comments).iter().enumerate() {
        let gliph = if index == 0 { 
            chunks.push(cformat!("\n <c>▌ Coments</>\n"));
            "─" 
        // } else { "╯" };
        } else { "─" };
        chunks.push(cformat!("  <b>╭{} </><green>{}</> <dim>- {} - <i>{}</>", gliph, comment.author.to_link(), cformat!("#{}", comment.id).as_link(&comment.url), comment.created_at.to_human()));
        chunks.push(lazy_parse!(errors, Some(comment.body.clone()), cformat!("<dim, i>no comment</>"), " "));
        
        if length == (index +1) { chunks.push(cformat!("  <b>╰──</> ")); }
        else {                 // chunks.push(cformat!("  <b>╰╮</> ")); 
                                  chunks.push(cformat!("  <b>╰──</> \n"));
            // chunks.push(cformat!("   <b>│</>"));
            // chunks.push(cformat!("   <b>│</>"));
        }
    }
    
    chunks.push(format!(""));
    
    (chunks, errors)
}
