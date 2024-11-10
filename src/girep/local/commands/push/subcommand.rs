use clap::{arg, command, Command};
use clap::builder::ValueParser;
use color_print::cformat;
use crate::cmdcore::args::Arguments;

pub(crate) fn push_subcommand() -> Command {
    command!("push").aliases(["p"])
        .about(cformat!("Interface to <b,i>git push</> using the given pconf"))
        .args([
            Arguments::repo_structure(false, true),
            Arguments::path(false, "Path to the repository"),
            arg!(    --all "The name of the branch"),
            arg!(    --branches "The name of the branch"),
            arg!(    --tags "The name of the branch"),
            arg!( -f --force "The name of the branch"),
            arg!( -n --dry-run "The name of the branch"),
            arg!( -u --set-uptream "The name of the branch"),
            arg!([remote] "The name of the remote"),
            arg!([branch] "The name of the branch"),
        ])
}