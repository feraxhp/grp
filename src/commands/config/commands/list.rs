use clap::{command, Command};

use crate::usettings::structs::Usettings;
use crate::system:: stdout;

pub fn command() -> Command {
    command!("list")
        .aliases(["ls"])
        .about("Shows the list of configured pconfs")
}

pub fn manager() {
    match Usettings::read() {
        Ok(u) => {
            for pconf in u.pconfs {
                stdout::writeln(pconf.name);
            }
        },
        Err(_) => todo!(),
    }
}
