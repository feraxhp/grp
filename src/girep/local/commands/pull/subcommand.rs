use clap::{arg, command, Arg, Command};
use color_print::cformat;
use crate::cmdcore::args::Arguments;

pub(crate) fn pull_subcommand() -> Command {
    command!("pull").aliases(["j"])
        .about(cformat!("Interface to <b,i>git pull</> using the given pconf"))
        .args([
            Arguments::pconf(false, true)
            ,
            arg!( -f --force "Do a force pull"),
            arg!( -n --"dry-run" "Do everything except actually send the updates."),
            arg!([remote] "The name of the remote to pull from"),
            arg!([branch] "The name of the branch to pull"),
            Arguments::path_flag(false, "Path to the repository"),
        ])
}
