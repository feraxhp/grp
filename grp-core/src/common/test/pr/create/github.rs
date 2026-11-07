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
use crate::Platform::Github;
use crate::animation;

#[tokio::test]
async fn create_github_pr() -> Result<(), mynox::Report> {
    let payload = json!({
        "title": "Amazing new feature",
        "body": "Please pull these awesome changes in!",
        "head": "octocat:new-topic",
        "base": "master",
        "draft": false,
        "maintainer_can_modify": true,
    });
    
    let server = server!(
        post!(201, "/repos/octocat/Hello-World/pulls", payload, value!("pr/create.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: "octocat".to_string(),
            repo: "Hello-World".to_string(),
            branch: "master".to_string(),
        },
        origin: BranchIdentifier {
            owner: "octocat".to_string(),
            repo: "Hello-World".to_string(),
            branch: "new-topic".to_string(),
        },
        title: "Amazing new feature".to_string(),
        body: "Please pull these awesome changes in!".to_string(),
        draft: false,
        maintainer_can_modify: true,
    };
    
    let created = Github.create_pr(&options, &config, &an).await?;
    
    assert_eq!(&created.title, "Amazing new feature");
    assert_eq!(&created.body, "Please pull these awesome changes in!");
    assert_eq!(created.number, 1347);
    assert_eq!(&created.author.name, "octocat");
    server.verify().await;
    Ok(())
}

#[tokio::test]
async fn create_github_pr_from_fork() -> Result<(), mynox::Report> {
    let payload = json!({
        "title": "Amazing new feature",
        "body": "Please pull these awesome changes in!",
        "head": "other_user:new-topic",
        "base": "master",
        "draft": false,
        "maintainer_can_modify": true,
    });
    
    let server = server!(
        post!(201, "/repos/octocat/Hello-World/pulls", payload, value!("pr/create.gh.json"), auth!(gh)),
    );
    
    let config = config!(gh server);
    let an = Box::new(animation::None);
    
    let options = PullRequestOptions {
        target: BranchIdentifier {
            owner: "octocat".to_string(),
            repo: "Hello-World".to_string(),
            branch: "master".to_string(),
        },
        origin: BranchIdentifier {
            owner: "other_user".to_string(),
            repo: "Hello-World".to_string(),
            branch: "new-topic".to_string(),
        },
        title: "Amazing new feature".to_string(),
        body: "Please pull these awesome changes in!".to_string(),
        draft: false,
        maintainer_can_modify: true,
    };
    
    let created = Github.create_pr(&options, &config, &an).await?;
    
    assert_eq!(&created.title, "Amazing new feature");
    assert_eq!(created.number, 1347);
    server.verify().await;
    Ok(())
}
