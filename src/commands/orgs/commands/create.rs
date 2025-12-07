use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};
use crate::animations::animation::Create;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::commands::validations::or_exit::structure::OrExit;
use crate::girep::animation::Animation;
use crate::girep::common::show::Show;
use crate::girep::platform::Platform;
use crate::girep::usettings::structs::{Pconf, Usettings};

pub fn command() -> Command {
    Commands::create("Create a new organization in the configured platform")
        .args([
            Arguments::pconf(true, true),
            arg!(<name> "The name of the org or path for groups creation")
            ,
            arg!(-r --recursive)
                .help(cformat!("Create groups recursively for <y>gitlab</>"))
                .required(false)
            ,
            arg!(-s  --"show-errors" "Show the erros when they happen during recursive orgs creation")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Create::new("Initializing org creation...");
    
    let recursive = args.get_flag("recursive");
    let show_errors = args.get_flag("show-errors");
    
    let pconf = match args.get_one::<Pconf>("pconf") {
        Some(e) => e.clone(),
        None => usettings.get_default_pconf().or_exit(&animation),
    };
    
    let name = args.get_one::<String>("name").unwrap();
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    let (users, errors) = platform.create_org(name, &config, recursive, &animation).await;
    
    match (users.is_empty(), errors.is_empty()) {
        (true, true) => {
            animation.finish_with_warning("No actions performed...");
        }
        (false, true) => {
            animation.finish_with_success(cformat!("<y,i>org creation</y,i> <g>succeeded!</>"));
            users.print_pretty();
        }
        (false, false) => {
            animation.finish_with_warning(cformat!("<m,i>org creation</m,i> <y>aborted!</>"));
            cprintln!("* Some groups where created!");
            users.print_pretty();
            if show_errors { errors.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        }
        (true, false) if errors.len() == 1  => {
            animation.finish_with_error(&errors[0].message);
            errors[0].show();
        }
        (true, false) => {
            animation.finish_with_error(cformat!("Something gone terribly wrong"));
            errors.print_pretty();
        }
    }
}