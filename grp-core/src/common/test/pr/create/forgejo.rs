use super::*;

use serde_json::json;
use crate::Config;
use crate::auth;
use crate::common::structs::BranchIdentifier;
use crate::common::structs::PullRequestOptions;
use crate::config;
use crate::server;
use crate::post;
use crate::value;
use crate::Platform::Forgejo;
use crate::animation;

#[tokio::test]
async fn create_forgejo_pr() -> Result<(), mynox::Report> {
    let payload = json!({
        "title": "Amazing new feature",
        "body": "Please pull these awesome changes in!",
        "head": "forgejo_user:new-topic",
        "base": "main",
    });
    
    let server = server!(
        post!(201, "/api/v1/repos/forgejo_user/Hello-Forgejo/pulls", payload, value!("pr/create.fg.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: "forgejo_user".to_string(),
            repo: "Hello-Forgejo".to_string(),
            branch: "main".to_string(),
        },
        origin: BranchIdentifier {
            owner: "forgejo_user".to_string(),
            repo: "Hello-Forgejo".to_string(),
            branch: "new-topic".to_string(),
        },
        title: "Amazing new feature".to_string(),
        body: "Please pull these awesome changes in!".to_string(),
        draft: false,
        maintainer_can_modify: true,
    };
    
    let created = Forgejo.create_pr(&options, &config, &an).await?;
    
    assert_eq!(&created.title, "Amazing new feature");
    assert_eq!(&created.body, "Please pull these awesome changes in!");
    assert_eq!(created.number, 1);
    assert_eq!(&created.author.name, "forgejo_user");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn create_forgejo_pr_from_fork() -> Result<(), mynox::Report> {
    let payload = json!({
        "title": "Amazing new feature",
        "body": "Please pull these awesome changes in!",
        "head": "other_user:new-topic",
        "base": "main",
    });
    
    let server = server!(
        post!(201, "/api/v1/repos/forgejo_user/Hello-Forgejo/pulls", payload, value!("pr/create.fg.json"), auth!(fj)),
    );
    
    let config = config!(fj server);
    let an = Box::new(animation::None);
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: "forgejo_user".to_string(),
            repo: "Hello-Forgejo".to_string(),
            branch: "main".to_string(),
        },
        origin: BranchIdentifier {
            owner: "other_user".to_string(),
            repo: "Hello-Forgejo".to_string(),
            branch: "new-topic".to_string(),
        },
        title: "Amazing new feature".to_string(),
        body: "Please pull these awesome changes in!".to_string(),
        draft: false,
        maintainer_can_modify: true,
    };
    
    let created = Forgejo.create_pr(&options, &config, &an).await?;
    
    assert_eq!(&created.title, "Amazing new feature");
    assert_eq!(created.number, 1);
    server.verify().await;
    Ok(())
}
