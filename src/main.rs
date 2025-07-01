// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod local;
mod system;
mod commands;
mod usettings;
mod animations;

use std::io;
use std::io::Write;
use std::process::exit;
use color_print::cprintln;
use clap::{arg, command, crate_version};

use crate::{commands::{config::config, core::common::invalid, local::{clone, pull, push}, orgs::orgs, repos::{create, delete, list}}, usettings::structs::Usettings};

#[tokio::main]
async fn main() {
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
            if *commands.get_one::<bool>("number").unwrap_or(&false) {
                let version = crate_version!();
                let _ = io::stdout().write(version.as_bytes());
                println!();
                exit(0);
            }
        },
        _ => {}
    }

    // match validate_version().await {
    //     Ok((true, version)) => { }
    //     Ok((false, version)) => {
    //         eprintln!("🎉 New version available!!");
    //         cprintln!("   → Latest  version: <g>{}</>", version.name.clone());
    //         cprintln!("   → Current version: <g>v{}</>", crate_version!());
    //         eprintln!();
    //         cprintln!("📥 Download it from: <b,u>{}</>", version.get_os_url());
    //         eprintln!();
    //     }
    //     Err(e) => {
    //         println!("{}", e.message);
    //         e.show();
    //     }
    // };

    let usettings = Usettings::read().unwrap_or_else(|e| {
        e.show();
        exit(1);
    });
    
    match commands.subcommand() {
        Some(sub) => match sub {
            ("config", args) => config::manager(args),
            ("list", args) => list::manager(args, usettings).await,
            ("create", args) => create::manager(args, usettings).await,
            ("delete", args) => delete::manager(args, usettings).await,
            ("orgs", args) => orgs::manager(args, usettings).await,
            ("clone", args) => clone::manager(args, usettings).await,
            ("push", args) => push::manager(args, usettings).await,
            ("pull", args) => pull::manager(args, usettings).await,
            _ => invalid(),
        },
        _ => {
            cprintln!("<y>* No command was provided try using <g,i>'--help'</>");
        }
    }
}
