use clap::Command;
use std::process::exit;
use color_print::{cprintln,cformat};
use clap::{ArgMatches, arg, command, crate_version};

use super::super::animations;
use super::super::commands::core::utils::version::show_version;
use super::super::girep::animation::Animation;
use super::super::girep::error::structs::Error;
use super::super::update::structs::Version;
use super::super::girep::usettings::structs::Usettings;
use super::super::commands::repos::{create, delete, list};
use super::super::commands::orgs::orgs;
use super::super::commands::local::{clone, pull, push};
use super::super::commands::core::common::invalid;
use super::super::commands::config::config;


pub fn command() -> Command {
    command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .arg(arg!( -v --"number" "Prints the version number to the standard output")
            .exclusive(true)
            .value_parser(show_version)
        )
        .subcommand(config::command())
        .subcommand(list::command())
        .subcommand(create::command())
        .subcommand(delete::command())
        .subcommand(orgs::command())
        .subcommand(clone::command())
        .subcommand(push::command())
        .subcommand(pull::command())
}

pub async fn mannager(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(sub) => {
            let usettings = Usettings::read().unwrap_or_else(|e| {
                cprintln!("* <y,i>usettings:</> <r>{}</>", e.message);
                e.show();
                exit(1);
            });
            let version = Version::validate_version();
            match sub {
                ("config", args) => config::manager(args),
                ("list", args) => list::manager(args, usettings).await,
                ("create", args) => create::manager(args, usettings).await,
                ("delete", args) => delete::manager(args, usettings).await,
                ("orgs", args) => orgs::manager(args, usettings).await,
                ("clone", args) => clone::manager(args, usettings).await,
                ("push", args) => push::manager(args, usettings).await,
                ("pull", args) => pull::manager(args, usettings).await,
                _ => invalid(),
            };
            let version = version.await;
            print_version(version, true);
        },
        _ => {
            let an = animations::animation::Process::new("Verifing version");
            let version = Version::validate_version().await;
            an.finish_with_warning(cformat!("No command was provided try using <g,i>'--help'</>"));
            print_version(version, false);
        }
    }
}

fn print_version(v:  Result<(bool, Version), Error>, new_line: bool) {
    match v {
        Ok((true, _)) => (),
        Ok((false, version)) => {
            if new_line { eprintln!(""); }
            eprintln!("🎉 New version available!!");
            cprintln!("   → Latest  version: <g>{}</>", version.name.clone());
            cprintln!("   → Current version: <g>v{}</>", crate_version!());
            eprintln!();
            cprintln!("📥 Download it from: <b,u>{}</>", version.get_os_url());
            eprintln!();
        }
        Err(_e) => { }
    };
}