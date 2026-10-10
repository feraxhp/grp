
use clap::{command, ArgMatches, Command};
use color_print::ceprintln;

use crate::{commands::core::{commands::Commands, common::invalid}, system::keiring::Keyring};


pub fn command() -> Command {
    command!("password")
        .aliases(["pass", "passw", "pasw", "psw"])
        .about("Password releated configuration")
        .subcommand(Commands::delete("Removes the entry from the keyring"))
        // .subcommand(add::command())
        // .subcommand(list::command())
        // .subcommand(cripto::command())
}

pub fn manager(args: &ArgMatches) {
    match args.subcommand() {
        Some(sub) => match sub {
            ("delete", _) => {
                match Keyring::forget() {
                    Ok(_) => ceprintln!("Password removed"),
                    Err(e) => ceprintln!("{}", e),
                };
            },
            // ("list", _) => list::manager(),
            // ("path" , _) => path::manager(),
            // ("cripto" , _) => cripto::manager(),
            _ => invalid()
        },
        _ => invalid()
    }
}