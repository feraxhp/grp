use serde::Serialize;
use mynox::Error;

use crate::specific::gitlab::groups::utils::search::get_closest_parent_and_stack;
use crate::common::structs::RequestType;
use crate::animation::Animation;
use crate::platform::Platform;
use crate::config::Config;
use crate::structs::User;

#[derive(Serialize)]
struct Group {
    parent_id: Option<String>,
    name: String,
    path: String,
}

#[allow(dead_code, unused_variables)]
pub(crate) async fn create_group<A: Animation + ?Sized>(
    platform: &Platform,
    name: &String,
    config: &Config,
    recursive: bool,
    animation: &Box<A>
) -> (Vec<User>, Vec<Error>) {
    animation.change_message("getting owned groups ...");
    let groups = match platform.get_logged_orgs(config).await {
        Ok(g) => g,
        Err(e) => {
            return (vec![], vec![e])
        },
    };
    
    let (mut parent, stack) = get_closest_parent_and_stack(name, &groups, recursive);
    let mut errors = vec![];
    let mut groups = vec![];
    
    for (i, name) in stack.iter().rev().enumerate() {
        animation.change_message(format!("Creating <m>{}/{}</m><y> groups ...</>", i, stack.len()));
        let p_ = match create(platform, &parent, name, config, animation).await {
            Ok(u) => {
                groups.push(u.clone());
                u
            },
            Err(e) => {
                errors.push(e);
                return (groups, errors);
            },
        };
        parent = Some(p_);
    }
    
    (groups, errors)
}

async fn create<A: Animation + ?Sized>(
    platform: &Platform,
    parent: &Option<User>,
    name: &str,
    config: &Config,
    animation: &Box<A>
) -> Result<User, Error> {
    assert!(matches!(platform, Platform::Gitlab));
    
    animation.change_message("generating url ...");
    let url = format!("{}/groups", platform.get_base_url(config));
    
    let group = Group {
        parent_id: parent.clone().map(|p| p.id),
        name: name.to_owned(),
        path: name.to_owned(),
    };
    
    let json = serde_json::to_value(group).unwrap();
    
    animation.change_message("creating group ...");
    let result = platform.post(url, true, config, &json).await?;
    
    animation.change_message("unwraping response ...");
    let base_message = "Error creating group ...";
    let result = platform.unwrap(result, base_message, config, &RequestType::CreateOrg).await?;
    
    User::from_text(&result, platform)
}
