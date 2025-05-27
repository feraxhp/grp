// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;
mod macros;
mod animations;
mod update;
mod commands;

use std::io;
use std::io::Write;
use std::process::exit;
use color_print::cprintln;
use clap::{arg, command, crate_version};

use crate::macros::macros::invalid;
use crate::update::os::base::Updater;
use crate::update::check::validate_version;
use crate::config::command::{config_command, config_manager};

use commands::local::clone::manager::clone_manager;
use commands::local::clone::subcommand::clone_subcommand;
use commands::local::push::subcommand::push_subcommand;
use commands::local::push::manager::push_manager;
use commands::local::pull::manager::pull_manager;
use commands::local::pull::subcommand::pull_subcommand;
use commands::repos::create::manager::create_manager;
use commands::repos::create::subcommand::create_subcommand;
use commands::repos::delete::manager::delete_manager;
use commands::repos::delete::subcommand::delete_subcommand;
use commands::repos::list::manager::list_manager;
use commands::repos::list::subcommand::list_subcommand;
use commands::orgs::command::orgs_command;
use commands::orgs::manager::orgs_manager;

#[tokio::main]
async fn main() {
    let commands = command!()
        .name("grp")
        .about("A simple CLI to manage platforms for git repositories")
        .arg(arg!( -v --"number" "Prints the version number to the standard output").exclusive(true))
        .subcommand(config_command())
        .subcommand(list_subcommand())
        .subcommand(create_subcommand())
        .subcommand(delete_subcommand())
        .subcommand(clone_subcommand())
        .subcommand(push_subcommand())
        .subcommand(pull_subcommand())
        .subcommand(orgs_command())
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

    match validate_version().await {
        Ok((true, version)) => { }
        Ok((false, version)) => {
            eprintln!("🎉 New version available!!");
            cprintln!("   → Latest  version: <g>{}</>", version.name.clone());
            cprintln!("   → Current version: <g>v{}</>", crate_version!());
            eprintln!();
            cprintln!("📥 Download it from: <b,u>{}</>", version.get_os_url());
            eprintln!();
        }
        Err(e) => {
            println!("{}", e.message);
            e.show();
        }
    };

    let user_settings = config::loader::load_configurations();

    match commands.subcommand() {
        Some(sub) => match sub {
            ("config", config) => config_manager(config),
            ("list", list) => list_manager(list, user_settings).await,
            ("create", create) => create_manager(create, user_settings).await,
            ("delete", delete) => delete_manager(delete, user_settings).await,
            ("clone", clone) => clone_manager(clone, user_settings).await,
            ("push", push) => push_manager(push, user_settings).await,
            ("pull", pull) => pull_manager(pull, user_settings).await,
            ("orgs", orgs) => orgs_manager(orgs, user_settings).await,
            _ => invalid()
        },
        _ => {
            cprintln!("<y>* No command was provided try using <g,i>'--help'</>");
        }
    }
}
