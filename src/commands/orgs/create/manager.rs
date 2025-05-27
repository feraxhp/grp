use std::process::exit;
use clap::ArgMatches;
use crate::girep::config::structure::Usettings;
use crate::girep::common::orgs::org::Org;
use crate::girep::platform::Platform;

pub(crate) async fn create_manager(create: &ArgMatches, usettings: Usettings) {
    let pconf = usettings.matches_pconf(create.get_one::<String>("pconf"));

    let name = match create.get_one::<String>("name") {
        Some(name) => name,
        None => {
            eprintln!("* You must provide a name for the organization");
            exit(1)
        }
    };

    let platform = Platform::matches(pconf.r#type.as_str());

    let org = match platform.create_org(name.clone(), pconf.to_conf()).await {
        Ok(org) => org,
        Err(e) => {
            e.show();
            exit(1)
        }
    };

    Org::show(vec![org])
}