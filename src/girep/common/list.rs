use clap::{arg, ArgMatches, Command};
use color_print::cprintln;

use crate::animations::animation::Fetch;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::girep::animation::Animation;
use crate::girep::common::show::Show;
use crate::girep::platform::Platform;
use crate::usettings::structs::Usettings;


pub fn command() -> Command {
    Commands::list("List the organizations for the authenticated user")
        .args([
            Arguments::pconf(false, true),
            arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing list organizations");
    
    let pconf = match args.get_one::<String>("pconf") {
        Some(e) if e == "-" => usettings.get_default_pconf().unwrap(),
        Some(e) => usettings.get_pconf_by_name(e).unwrap(),
        None => usettings.get_default_pconf().unwrap(),
    };
    
    
    let show_errors = args.get_flag("show-errors");
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    let (orgs, _pag_error, _errors) = platform.list_orgs(&config, &animation).await;
    
    match (orgs, _pag_error, _errors) {
        (o, None, e) if e.is_empty() && !o.is_empty() => {
            animation.finish_with_success("Organizations listed successfully");
            o.print_pretty();
        },
        (o, None, e) if e.is_empty() && o.is_empty() => {
            animation.finish_with_success("No organizations found");
        },
        (_, Some(e), _) => {
            animation.finish_with_error(format!("{}", e.message));
            e.show();
        },
        (o, None, e) if !o.is_empty() && !e.is_empty() => {
            animation.finish_with_warning("Organizations listed with errors");
            o.print_pretty();
            if show_errors { e.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        },
        _ => unreachable!()
    }
}