// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::config::structure::{Pconf, Usettings};
use crate::girep::platform::Platform;
use crate::show;
use clap::ArgMatches;

pub(crate) async fn list_manager(clist: &ArgMatches, usettings: Usettings) {

    let pconf = usettings.matches_pconf(clist.get_one::<String>("pconf"));

    let platform = Platform::matches(pconf.r#type.as_str());

    let repos = match clist.get_one::<String>("owner") {
        Some(owner) => platform.list_repos(Some(owner.to_string()), pconf.to_conf()).await,
        None => platform.list_repos(None, pconf.to_conf()).await
    };

    show!(repos.0);
}
