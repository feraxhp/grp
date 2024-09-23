// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::config::structure::Usettings;
use crate::girep::repos::common::supported::Platform;
use crate::show;
use clap::ArgMatches;

pub(crate) async fn list_manager(clist: &ArgMatches, usettings: Usettings) {

    let pconf = match clist.get_one::<String>("pconf"){
        Some(clist) => {
            match usettings.get_pconf(clist.clone()) {
                Some(pconf) => pconf,
                None => {
                    let pconf = usettings.get_default();
                    if !clist.eq("-") {
                        eprintln!("{} is not a valid pconf name", clist);
                        eprintln!("using default pconf: {}", pconf.name.clone());
                    }
                    pconf.clone()
                },
            }
        },
        None => { usettings.get_default() }
    };

    let platform = Platform::matches(pconf.r#type.as_str());

    let repos = match clist.get_one::<String>("owner") {
        Some(owner) => platform.list_repos(Some(owner.to_string()), pconf.to_conf()).await,
        None => platform.list_repos(None, pconf.to_conf()).await
    };

    show!(repos.0);
}
