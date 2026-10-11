use clap::{command, ArgMatches, Command};
use color_print::{ceprintln, cprintln};

use crate::{fail, success};
use crate::system::show::Empty;
use crate::usettings::structs::{Pconf, Usettings};
use crate::commands::core::common::invalid;
use crate::commands::core::commands::Commands;
use crate::commands::core::args::Arguments;


pub fn command() -> Command {
    command!("default")
        .aliases(["def"])
        .about("Sets or shows the default pconf")
        .args([
            Arguments::pconf(false, false),
        ])
        .subcommand(Commands::list("List all the pconfs, selecting the deafult"))
}

pub fn manager(args: &ArgMatches) {
    let mut usettings = match Usettings::read() {
        Ok(us) => us,
        Err(e) => fail!(e),
    };
    
    match args.subcommand() {
        Some(sub) => match sub {
            ("list", _) => {
                let default = match usettings.get_raw_default_pconf() {
                    Some(p) => &p.name,
                    None => {
                        let default = &usettings.default;
                        ceprintln!("<y>orphan default</> '{}'", default.empty());
                        default
                    },
                };
                
                for pconf in usettings.pconfs.iter() {
                    match &pconf.name == default {
                        true  => cprintln!("<g    >{} <m><<- default </>", pconf.name),
                        false => cprintln!("<dim,i>{}</>", pconf.name),
                    }
                }
            },
            _ => invalid()
        },
        _ => {
            
            let pconf = match args.get_one::<Pconf>("pconf") {
                Some(e) => e,
                None => unreachable!(),
            };
            
            usettings.default = pconf.name.clone();
            match usettings.save() {
                Ok(_) => success!("set default pconf"),
                Err(e) => fail!(e),
            }
        }
    }
}
