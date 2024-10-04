// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::ArgMatches;
use crate::config::structure::Usettings;
use crate::girep::common::orgs::subcommands::list::manager::list_manager;
use crate::macros::macros::invalid;

pub(crate) async fn orgs_manager(orgs: &ArgMatches, usettings: Usettings){
    match orgs.subcommand() {
        Some(sub) => match sub {
            ("list", clist) => list_manager(clist, usettings).await,
            // ("default", default) => default_manager(default),
            _ => invalid()
        },
        _ => invalid()

    }
}
