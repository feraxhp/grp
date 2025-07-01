
use std::io;

use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};
use crate::animations::animation::Delete;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::core::utils::repo_struct::unfold_repo_structure;
use crate::girep::animation::Animation;
use crate::girep::platform::Platform;
use crate::usettings::structs::Usettings;

pub fn command() -> Command {
    Commands::delete("Delete a repository")
        .args([
            Arguments::repo_structure(true, true),
            arg!(-y --yes "Skip the confirmation prompt")
                .required(false)
            ,
            arg!(-s --soft)
                .help(cformat!("just <i>mark</> the org for delition <y>[gitlab]</>"))
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Delete::new("Initializing repository deletion...");
    
    let srepo = args.get_one::<String>("repo").unwrap();
    let (pconf, owner, repo) = unfold_repo_structure(srepo.as_str(), false, &usettings).unwrap();

    let pconf = match pconf {
        Some(e) if e != "*" => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        _ => {
            cprintln!("<y>For security reasons you have to proviede explicitly the <m>pconf name</>");
            return;
        },
    };
    
    let soft = args.get_flag("soft");
    let mut confirmation = args.get_flag("yes");
    
    if !confirmation {
        eprintln!(
            "Do you realy whant to delete {}:{}/{}?",
            pconf.name, owner, repo
        );
    }
    
    while !confirmation {
        eprint!(
            "Type '{}/{}' to confirm: ",
            owner, repo
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Something went wrong while reading the input");
        let input = input.trim();

        confirmation = input == format!("{}/{}", owner, repo);
    }
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    match platform.delete_repo(owner, repo, &config, !soft, Some(&animation)).await {
        Ok(_) => {
            let message = match (soft, platform) {
                (true, Platform::Gitlab) => cformat!("repo <m,i>mark</> <g>for delition!</>"),
                (true, _) => vec![
                    cformat!("<y,i>repo delition</y,i> <g>succeeded!</>"),
                    cformat!("<y>* <i>soft delition</i> only wotks with <m>gitlab</>")
                ].join("\n"),
                (false, _) => cformat!("<y,i>repo delition</y,i> <g>succeeded!</>"),
            };
            
            animation.finish_with_success(message);
        },
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
        },
    };
}