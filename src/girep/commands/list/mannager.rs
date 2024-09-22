use crate::config::structure::Usettings;
use crate::girep::repos::common::supported::Platform;
use crate::show;
use clap::ArgMatches;
use color_print::cprintln;
use std::process::exit;

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

    let platform = match pconf.r#type.as_str() {
        "github" => Platform::Github,
        "gitea" => Platform::Gitea,
        _ => {
            cprintln!("* Error: <i>{}</> is not a valid platform", pconf.r#type.clone());
            exit(1)
        }
    };

    let repos = match clist.get_one::<String>("owner") {
        Some(owner) => platform.list_repos(Some(owner.to_string()), pconf.to_conf()).await,
        None => platform.list_repos(None, pconf.to_conf()).await
    };

    show!(repos.0);
}
