// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::structure::{Pconf, Usettings};
use crate::girep::platform::Platform;
use crate::show;
use clap::ArgMatches;
use color_print::cprintln;

pub(crate) async fn list_manager(clist: &ArgMatches, usettings: Usettings) {

    let pconf = usettings.matches_pconf(clist.get_one::<String>("pconf"));

    let platform = Platform::matches(pconf.r#type.as_str());

    let repos = match clist.get_one::<String>("owner") {
        Some(owner) => platform.list_repos(Some(owner.to_string()), pconf.to_conf()).await,
        None => platform.list_repos(None, pconf.to_conf()).await
    };

    if(repos.1.len() > 0) {
        for error in repos.1 {
            cprintln!("<r>→ Error:</> {}", error.message);
            error.content.iter().for_each(|inter| {
                println!("  {}", inter);
            })
        }
    }

    show!(repos.0);
}
