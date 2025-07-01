
use std::io;

use clap::{arg, ArgMatches, Command};
use color_print::cformat;

use crate::girep::platform::Platform;
use crate::girep::animation::Animation;
use crate::animations::animation::Delete;
use crate::usettings::structs::Usettings;
use crate::commands::core::args::Arguments;
use crate::commands::core::commands::Commands;

pub fn command() -> Command {
    Commands::delete("Delete an organization")
        .args([
            Arguments::pconf(true, false),
            arg!(<name> "The name of the org or path for group deletion")
            ,
            arg!(-y --yes "Skip the confirmation prompt")
                .required(false)
            ,
            arg!(-s --soft)
                .help(cformat!("just <i>mark</> the org for delition <y>[gitlab]</>"))
                .required(false)
        ])
}

pub async fn manager(args: &ArgMatches, usettings: Usettings) {
    let animation = Delete::new("Initializing organization deletion...");
    
    let pconf = match args.get_one::<String>("pconf") {
        Some(e) if e == "-" => usettings.get_default_pconf().unwrap(),
        Some(e) => usettings.get_pconf_by_name(e).unwrap(),
        None => usettings.get_default_pconf().unwrap(),
    };
    
    let name = args.get_one::<String>("name").unwrap();
    let soft = args.get_flag("soft");
    let mut confirmation = args.get_flag("yes");
    
    if !confirmation {
        eprintln!(
            "Do you realy whant to delete {} {}?",
            pconf.name, &name
        );
    }
    
    while !confirmation {
        eprint!("Type '{}' to confirm: ", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Something went wrong while reading the input");
        let input = input.trim();

        confirmation = input == format!("{}", name);
    }
    
    let platform = Platform::matches(pconf.r#type.as_str());
    let config = pconf.to_config();
    
    match platform.delete_org(name, &config, !soft, Some(&animation)).await {
        Ok(_) => {
            let message = match (soft, platform) {
                (true, Platform::Gitlab) => cformat!("group <m,i>marked</> <g>for delition!</>"),
                (true, _) => vec![
                    cformat!("<y,i>org delition</y,i> <g>succeeded!</>"),
                    cformat!("<y>* <i>soft delition</i> only wotks with <m>gitlab</>")
                ].join("\n"),
                (false, Platform::Gitlab) => cformat!("<y,i>group delition</y,i> <g>succeeded!</>"),
                (false, _) => cformat!("<y,i>org delition</y,i> <g>succeeded!</>"),
            };
            
            animation.finish_with_success(message);
        },
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
        },
    };
}