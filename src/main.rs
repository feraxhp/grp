// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod local;
mod system;
mod update;
mod commands;
mod usettings;
mod animations;

use std::io;
use std::io::Write;
use std::process::exit;
use color_print::{cprintln,cformat};
use clap::{arg, command, crate_version};

use crate::girep::animation::Animation;
use crate::girep::error::structs::Error;
use crate::update::structs::Version;
use crate::usettings::structs::Usettings;
use crate::commands::repos::{create, delete, list};
use crate::commands::orgs::orgs;
use crate::commands::local::{clone, pull, push};
use crate::commands::core::common::invalid;
use crate::commands::config::config;

#[tokio::main]
async fn main() {
    let an = animations::animation::Process::new("Reading input");
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .arg(arg!( -v --"number" "Prints the version number to the standard output").exclusive(true))
        .subcommand(config::command())
        .subcommand(list::command())
        .subcommand(create::command())
        .subcommand(delete::command())
        .subcommand(orgs::command())
        .subcommand(clone::command())
        .subcommand(push::command())
        .subcommand(pull::command())
        .get_matches();


    match commands.clone().args_present() {
        true => {
            an.spinner.finish_and_clear();
            if *commands.get_one::<bool>("number").unwrap_or(&false) {
                let version = crate_version!();
                let _ = io::stdout().write(version.as_bytes());
                println!();
                exit(0);
            }
        },
        _ => {}
    }
    
    match commands.subcommand() {
        Some(sub) => {
            an.change_message("Reading user settings");
            let usettings = Usettings::read().unwrap_or_else(|e| {
                an.finish_with_error(cformat!("<y,i>usettings:</> <r>{}</>", e.message));
                e.show();
                exit(1);
            });
            let version = Version::validate_version();
            an.spinner.finish_and_clear();
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
            an.change_message("Verifing version");
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