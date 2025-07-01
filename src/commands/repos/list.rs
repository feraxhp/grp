use clap::{arg, ArgMatches, Command};
use color_print::{cformat, cprintln};

use crate::animations::animation::Fetch;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;
use crate::girep::animation::Animation;
use crate::girep::common::show::Show;
use crate::girep::platform::Platform;
use crate::usettings::structs::Usettings;

pub fn command() -> Command {
    Commands::list("List the all the repositories from a configured repo and the given user")
        .args([
            Arguments::pconf(false, true),
            arg!([owner] "The user to list the repositories from"),
            arg!(-s  --"show-errors" "Show the erros when they happen during paggination request")
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Fetch::new("Incializing list repositories");
    
    let pconf = match args.get_one::<String>("pconf") {
        Some(e) if e == "-" => usettings.get_default_pconf().unwrap(),
        Some(e) => usettings.get_pconf_by_name(e).unwrap(),
        None => usettings.get_default_pconf().unwrap(),
    };
    
    let show_errors = args.get_flag("show-errors");
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    let (repos, _pag_error, _errors) = platform.list_repos(args.get_one::<String>("owner"), &config, Some(&animation)).await;
    
    match (repos, _pag_error, _errors) {
        (r, None, e) if e.is_empty() && !r.is_empty() => {
            animation.finish_with_success(cformat!("<y,i>list repos</y,i> <g>succeeded!</>"));
            r.print_pretty();
        },
        (r, None, e) if e.is_empty() && r.is_empty() => {
            animation.finish_with_success("<i>No repos found</>");
        },
        (_, Some(e), _) => {
            animation.finish_with_error(format!("{}", e.message));
            e.show();
        },
        (r, None, e) if !r.is_empty() && !e.is_empty() => {
            animation.finish_with_warning(cformat!("<m,i>list repos</m,i> <y>finish with errors!</>"));
            r.print_pretty();
            if show_errors { e.print_pretty(); } 
            else {
                cprintln!("<y>* Some errors were found, use <g,i>--show-errors</g,i> to see them</>");
            }
        },
        _ => unreachable!()
    }
}
