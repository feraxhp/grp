use color_print::cformat;
use serde::Serialize;

use crate::girep::common::structs::{Context, RequestType};
use crate::girep::common::users::structs::User;
use crate::girep::config::Config;
use crate::girep::error::structs::Error;
use crate::girep::platform::Platform;
use crate::girep::animation::Animation;
use crate::girep::specific::gitlab::groups::utils::search::get_closest_parent_and_stack;

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
    animation: Option<&Box<A>>
) -> (Vec<User>, Vec<Error>) {
    if let Some(an) = animation { an.change_message("getting owned groups ..."); }
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
        if let Some(an) = animation { an.change_message(cformat!("Creating <m>{}/{}</m><y> groups ...</>", i, stack.len())); }
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
    name: &String,
    config: &Config,
    animation: Option<&Box<A>>
) -> Result<User, Error> {
    assert!(matches!(platform, Platform::Gitlab));
    
    if let Some(an) = animation { an.change_message("generating url ..."); }
    let url = format!("{}/groups", platform.get_base_url(&config.endpoint));
    
    let group = Group {
        parent_id: parent.clone().map(|p| p.id),
        name: name.clone(),
        path: name.clone(),
    };
    
    let json = serde_json::to_value(group).unwrap();
    
    if let Some(an) = animation { an.change_message("creating group ..."); }
    let result = platform.post(url, true, config, &json).await?;
    
    if let Some(an) = animation { an.change_message("unwraping response ..."); }
    let context = Context {
        request_type: RequestType::CreateOrg,
        owner: None, repo: None,
        additional: Some(name.clone()),
    };
    
    let base_message = "Error creating group ...";
    let result = platform.unwrap(result, base_message, config, context).await?;
    
    User::from_text(&result, platform)
}
