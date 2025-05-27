use crate::config::structure::Usettings;
use crate::girep::platform::Platform;
use crate::animations;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::animations::animation::Animation;
use crate::errors::error::Error;
use crate::girep::local::errors::Action;
use crate::girep::local::git_utils::options::{Methods, Options};

pub(crate) async fn push_manager(cpush: &ArgMatches, usettings: Usettings) {

    let load_animation = animations::creation::Create::new("Pushing repository ...");

    let pconf = usettings.matches_pconf(cpush.get_one::<String>("pconf"));
    let all = cpush.get_one::<bool>("all").unwrap();
    let branches = cpush.get_one::<bool>("branches").unwrap();
    let tags = cpush.get_one::<bool>("tags").unwrap();
    let force = cpush.get_one::<bool>("force").unwrap();
    let dry_run = cpush.get_one::<bool>("dry-run").unwrap();

    let path = match cpush.get_one::<PathBuf>("path"){
        Some(value) => value.clone(),
        None => std::env::current_dir().unwrap()
    };

    let mut remote = match cpush.get_one::<String>("remote") {
        None => None,
        Some(e) => Some(e.to_owned())
    };

    let mut branch = match cpush.get_one::<String>("branch") {
        None => None,
        Some(e) => Some(e.to_owned())
    };

    let up_stream = if let Some(values) = cpush.get_many::<String>("set-upstream") {
        let mut values_iter = values.clone();
        remote = Some(values_iter.next().unwrap().to_owned());
        branch = Some(values_iter.next().unwrap().to_owned());
        true
    } else { false };

    let platform = Platform::matches(pconf.r#type.as_str());

    let method = match "" {
        _ if *all => Methods::ALL,
        _ if *branches => Methods::BRANCHES,
        _ if *tags => Methods::TAGS,
        _ if up_stream => Methods::UPSTREAM,
        _ => Methods::DEFAULT
    };

    let options = Options {
        method,
        remote,
        branch,
        force: *force,
        dry_run: *dry_run
    };

    let result = platform.push_repo(&path, options, &usettings);

    match result {
        Ok(messages) => {
            load_animation.finish_with_success(messages.last().unwrap().as_str());
            for (i, e) in messages.iter().enumerate() {
                if i == messages.len() - 1 { break; }
                eprintln!("{}", e);
            }
        }
        Err(e) => {
            let error = Error::git_to_local(e, path, usettings.get_default().to_conf(), Action::PUSH);
            load_animation.finish_with_error(error.message.as_str());
            error.show();
        }
    }
}