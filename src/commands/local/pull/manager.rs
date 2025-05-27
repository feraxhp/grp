use crate::animations;
use crate::animations::animation::Animation;
use crate::config::structure::Usettings;
use crate::girep::errors::error::Error;
use crate::girep::local::errors::Action;
use crate::girep::local::git_utils::options::{Methods, Options};
use crate::girep::platform::Platform;
use clap::ArgMatches;
use std::path::PathBuf;

pub(crate) async fn pull_manager(cpull: &ArgMatches, usettings: Usettings) {

    let load_animation = animations::fetch::Fetch::new("Fetching the repository ...");

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
        method: Methods::DEFAULT,
        remote,
        branch,
        force: *force,
        dry_run: *dry_run
    };
    let result = platform
        .fetch_repo(
            &path, options,
            &usettings, true,
            |s: String| { load_animation.change_message(s) }
        );

    match result {
        Ok((messages, commit)) => {
            load_animation.finish_with_success(messages.last().unwrap().as_str());
            for (i, e) in messages.iter().enumerate() {
                if i == messages.len() - 1 { break; }
                eprintln!("{}", e);
            }
        }
        Err(e) => {
            let error = Error::git_to_local(e, path, usettings.get_default().to_conf(), Action::PULL);
            load_animation.finish_with_error(error.message.as_str());
            error.show();
        }
    }
}