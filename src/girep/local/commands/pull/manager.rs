use crate::config::structure::Usettings;
use crate::girep::platform::Platform;
use crate::animations;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::animations::animation::Animation;
use crate::girep::local::pull::Options;

pub(crate) async fn pull_manager(cpull: &ArgMatches, usettings: Usettings) {

    let load_animation = animations::fetch::Fetch::new("Pushing repository ...");

    let pconf = usettings.matches_pconf(cpull.get_one::<String>("pconf"));
    let force = cpull.get_one::<bool>("force").unwrap();
    let dry_run = cpull.get_one::<bool>("dry-run").unwrap();

    let mut branch = match cpull.get_one::<String>("branch") {
        None => None,
        Some(e) => Some(e.to_owned())
    };

    let mut remote = match cpull.get_one::<String>("remote") {
        None => None,
        Some(e) => Some(e.to_owned())
    };

    let path = match cpull.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap()
    };

    let platform = Platform::matches(pconf.r#type.as_str());

    let options = Options {
        remote,
        branch,
        force: *force,
        dry_run: *dry_run
    };

    // let result = platform.push_repo(path, options, pconf.to_conf());
    //
    // match result {
    //     Ok(messages) => {
    //         load_animation.finish_with_success(messages.last().unwrap().as_str());
    //         for (i, e) in messages.iter().enumerate() {
    //             if i == messages.len() - 1 { break; }
    //             eprintln!("{}", e);
    //         }
    //     }
    //     Err(e) => {
    //         load_animation.finish_with_error(e.message.as_str());
    //         e.show();
    //     }
    // }
}