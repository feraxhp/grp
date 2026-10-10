use std::process::exit;

use clap::{ArgMatches, Command, arg, command};
use color_print::cformat;
use grp_core::animation::Animation;

use crate::usettings::structs::Pconf;
use crate::usettings::structs::Usettings;
use crate::animations::animation::Process;
use crate::commands::core::args::Arguments;


pub fn command() -> Command {
    command!("encript")
        .aliases(["lock"])
        .about("Encripts the given pconf")
        .args([
            Arguments::pconf(false, false)
                .required_unless_present("all")
            ,
            arg!(-a --all "Encripts all the pconfs")
        ])
}

pub fn manager(args: &ArgMatches) {
    let mut animation = Process::new("Reading user configuration...");
    let all = args.get_flag("all");
    let mut usettings = match Usettings::read() {
        Ok(us) => us,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1);
        },
    };
    
    let password = match usettings.get_password(true, &animation) {
        Ok(ps) => ps,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1);
        },
    };
    
    let outcome = match all {
        true => {
            animation.change_message("encripting all pconfs");
            usettings.cipher_all_pconfs(&password, &mut animation)
        },
        false => {
            let pconf = args.get_one::<Pconf>("pconf").unwrap();
            animation.change_message(cformat!("encripting pconf <i>{}</>", pconf.name));
            usettings.cipher_pconf(&pconf.name, &password)
        },
    };
    
    let error = match outcome {
        Ok(true) => {
            match usettings.save() {
                Ok(_) => {
                    usettings.try_safe_password(&password);
                    animation.finish_with_success(cformat!("<y,i>encription</y,i> <g>succeeded!</>"));
                    None
                },
                Err(e) => Some(e),
            }
        },
        Ok(false) => {
            animation.finish_with_warning("nothing to do");
            None
        },
        Err(e) => Some(e),
    };
    
    if let Some(error) = error {
        animation.finish_with_error(&error.message);
        error.show();
    }
}