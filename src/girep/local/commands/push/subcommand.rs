use clap::{arg, command, Arg, Command};
use color_print::cformat;
use crate::cmdcore::args::Arguments;

pub(crate) fn push_subcommand() -> Command {
    command!("push").aliases(["p"])
        .about(cformat!("Interface to <b,i>git push</> using the given pconf"))
        .args([
            Arguments::pconf(false, true),
            arg!(    --all "Push all branches")
                .conflicts_with_all(["branches", "tags", "set-upstream", "branch"])
            ,
            arg!(    --branches "Push all branches")
                .conflicts_with_all(["all", "tags", "set-upstream", "branch"])
            ,
            arg!(    --tags "Push all tags")
                .conflicts_with_all(["all", "branches", "set-upstream", "branch"])
            ,
            arg!( -f --force "Do a force push"),
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
