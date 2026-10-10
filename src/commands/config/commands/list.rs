use clap::{command, Command};
use color_print::ceprintln;

use crate::system::show::Show;
use crate::usettings::structs::Usettings;


pub fn command() -> Command {
    command!("list")
        .aliases(["ls"])
        .about("Shows the list of configured pconfs")
}

pub fn manager() {
    match Usettings::read() {
        Ok(u) => u.pconfs.print_pretty(),
        Err(e) => {
            ceprintln!("{}", &e.message);
            e.show();
        },
    }
}
