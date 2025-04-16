// Copyright 2024 feraxhp
// Licensed under the MIT License;

mod girep;
mod config;
mod macros;
mod animations;
mod errors;
mod cmdcore;
mod update;

use crate::config::command::{config_command, config_manager};
use crate::girep::common::orgs::command::orgs_command;
use crate::girep::common::orgs::manager::orgs_manager;
use crate::macros::macros::invalid;
use crate::update::check::validate_version;
use crate::update::os::base::Updater;
use clap::{arg, command, crate_version};
use color_print::cprintln;
use girep::local::commands::clone::manager::clone_manager;
use girep::local::commands::clone::subcommand::clone_subcommand;
use girep::local::commands::push::subcommand::push_subcommand;
use girep::local::commands::push::manager::push_manager;
use girep::common::repos::commands::create::manager::create_manager;
use girep::common::repos::commands::create::subcommand::create_subcommand;
use girep::common::repos::commands::delete::manager::delete_manager;
use girep::common::repos::commands::delete::subcommand::delete_subcommand;
use girep::common::repos::commands::list::manager::list_manager;
use girep::common::repos::commands::list::subcommand::list_subcommand;
use std::io;
use std::io::Write;
use std::process::exit;
use crate::girep::local::commands::pull::manager::pull_manager;
use crate::girep::local::commands::pull::subcommand::pull_subcommand;

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
