use clap::{command, Command};

use crate::system::{directories::Directories, stdout};

pub fn command() -> Command {
    command!("path")
        // .aliases([""])
        .about("Shows the realpath for the configurations")
}

pub fn manager() {
    match Directories::config_dir() {
        Ok(path) => {
            let string = path.as_os_str().to_str();
            match string {
                Some(s) => stdout::writeln(s),
                None => todo!(),
            }
        },
        Err(_) => todo!(),
    };
}
