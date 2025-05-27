use std::io::{self, Write};

use clap::{command, Command};
use color_print::cprint;

use crate::config::location::get_location;

pub fn path_manager() {
    cprint!("<g>* Configuration path in: </><b,i>");
    let _ = io::stdout().write(get_location().as_bytes());
    let _ = io::stdout().flush();
    print!("\n");
}

pub fn path_command() -> Command {
    command!("path")
        .aliases(["pth", "p"])
        .about("Print the config path for grp to the standard output")
}