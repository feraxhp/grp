use clap::ArgMatches;
use hyper::Error;
use crate::config::structure::{Pconf, Usettings};
use crate::girep::common::orgs::org::Org;
use crate::girep::platform::Platform;

pub(crate) async fn list_manager(clist: &ArgMatches, usettings: Usettings) {
    let pconf = usettings.matches_pconf(clist.get_one::<String>("pconf"));

    let platform = Platform::matches(pconf.r#type.as_str());

    let (orgs, errors) = platform.list_orgs(pconf.to_conf()).await;

    if(!errors.is_empty()) {
        errors.iter().for_each(
            |e| {
                e.show();
            }
        )
    }

    Org::show(orgs)
}