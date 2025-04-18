use clap::{arg, command, Arg, Command};
use color_print::cformat;
use crate::cmdcore::args::Arguments;

pub(crate) fn pull_subcommand() -> Command {
    command!("pull").aliases(["j"])
        .about(cformat!("Interface to <b,i>git pull</> using the given pconf"))
        .args([
            Arguments::pconf(false, true),
            arg!( -f --force "Do a force pull"),
            arg!( -n --"dry-run" "Do everything except actually send the updates."),
            Arg::new("set-upstream").short('u').long("set-upstream")
                .num_args(2)
                .value_names(["remote", "branch"])
                .conflicts_with_all(["all", "branches", "tags"])
                .help("Sets the name of the remote as default upstream for a branch"),
            arg!([remote] "The name of the remote to push to")
                .conflicts_with("set-upstream"),
            arg!([branch] "The name of the branch to push")
                .conflicts_with("set-upstream"),
            Arguments::path_flag(false, "Path to the repository"),
        ])
}