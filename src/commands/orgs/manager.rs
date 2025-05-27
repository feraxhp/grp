// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::ArgMatches;
use crate::girep::macros::macros::invalid;
use crate::girep::config::structure::Usettings;
use crate::commands::orgs::list::manager::list_manager;
use crate::commands::orgs::create::manager::create_manager;
use crate::commands::orgs::delete::manager::delete_manager;

pub(crate) async fn orgs_manager(orgs: &ArgMatches, usettings: Usettings){
    match orgs.subcommand() {
        Some(sub) => match sub {
            ("list", clist) => list_manager(clist, usettings).await,
            ("create", create) => create_manager(create, usettings).await,
            ("delete", delete) => delete_manager(delete, usettings).await,
            _ => invalid()
        },
        _ => invalid()

    }
}
