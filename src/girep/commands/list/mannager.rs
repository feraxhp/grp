use clap::ArgMatches;
use crate::config::structure::Usettings;
use crate::girep::repos::platform::get_platform;
use crate::show;

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

    let platform = get_platform(pconf.clone()).await;

    let repos = match clist.get_one::<String>("owner") {
        Some(owner) => platform.list_repos(Some(owner.clone())).await,
        None => platform.list_repos(None).await
    };

    show!(repos);
}
