
use std::io;
use std::process::exit;

use clap::{arg, ArgMatches, Command};
use color_print::cformat;
use crate::animations::animation::Delete;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::repo::RepoStructure;
use grp_core::animation::Animation;
use grp_core::platform::Platform;
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
    
    let repo = args.get_one::<RepoStructure>("repo").unwrap();
    
    let pconf = match repo.pconf.clone() {
        Some(e) 
        if !matches!(usettings.get_pconf_by_name(e.as_str()), None) 
        => usettings.get_pconf_by_name(e.as_str()).unwrap(),
        
        _ => {
            animation.finish_with_error(cformat!("For security reasons you have to proviede explicitly the <m>pconf name</>"));
            return;
        },
    };
    
    let soft = args.get_flag("soft");
    let mut confirmation = args.get_flag("yes");
    
    if !confirmation {
        eprintln!(
            "Do you realy whant to delete {}:{}/{}?",
            pconf.name, &repo.owner, &repo.path
        );
    }
    
    while !confirmation {
        eprint!(
            "Type '{}/{}' to confirm: ",
            &repo.owner, &repo.path
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Something went wrong while reading the input");
        let input = input.trim();

        confirmation = input == format!("{}/{}", &repo.owner, &repo.path);
    }
    
    let platform = Platform::matches(pconf.r#type.as_str());
    if let Err(e) = repo.is_unsupported(&platform) {
        animation.finish_with_error(&e.message);
        e.show();
        exit(1)
    }
    
    let config = pconf.to_config();
    
    match platform.delete_repo(&repo.owner, &repo.path, &config, !soft, &animation).await {
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