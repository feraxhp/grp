use std::process::exit;

use clap::{ArgMatches, Command, command};
use color_print::cformat;
use grp_core::Formater;
use grp_core::animation::Animation;

use crate::animations::animation::Suspend;
use crate::usettings::structs::Usettings;
use crate::animations::animation::Process;


pub fn command() -> Command {
    command!("change")
        // .aliases(["unlock"])
        .about("Changes the password used for the encription")
}

pub fn manager(_: &ArgMatches) {
    let mut animation = Process::new("Reading user configuration...");
    let mut usettings = match Usettings::read() {
        Ok(us) => us,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1);
        },
    };

    let ns = Usettings { default: String::new(), keyring: false, hidepass: usettings.hidepass.clone(), pconfs: vec![]  };
    _ = animation.multi.println(cformat!("<y,i>insert</y,i> <m,i>old password</>").as_tip());
    let old = match ns.get_password(false, &animation) {
        Ok(ps) => ps,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1);
        },
    };
    animation.suspend(||{ eprint!("\x1B[1A\x1B[0J"); });

    _ = animation.multi.println(cformat!("<y,i>insert</y,i> <m,i>new password</>").as_tip());
    let new = match ns.get_password(true, &animation) {
        Ok(ps) => ps,
        Err(e) => {
            animation.finish_with_error(&e.message);
            e.show();
            exit(1);
        },
    };
    animation.suspend(||{ eprint!("\x1B[1A\x1B[0J"); });
    
    animation.change_message("changing password...");
    let outcome = usettings.change_password(&old, &new, &mut animation);

    let error = match outcome {
        Ok(true) => {
            match usettings.save() {
                Ok(_) => {
                    usettings.try_safe_password(&new);
                    animation.finish_with_success(cformat!("<y,i>change password</y,i> <g>succeeded!</>"));
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
