use clap::ArgMatches;
use hyper::Error;
use crate::config::structure::Usettings;
use crate::girep::common::orgs::org::Org;
use crate::girep::platform::Platform;

pub(crate) async fn list_manager(clist: &ArgMatches, usettings: Usettings) {
    let pconf = match clist.get_one::<String>("pconf") {
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