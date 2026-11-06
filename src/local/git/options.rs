use git2::Error;

use crate::local::git::utils::GitUtils;


#[derive(PartialEq, Clone)]
pub(crate) enum Methods {
    All,
    Tag(String),
    Tags,
    Default,
    Branches,
    Upstream
}

#[derive(Clone)]
pub(crate) struct Options {
    pub(crate) method: Methods,
    pub(crate) remote: Option<String>,
    pub(crate) branch: Option<String>,
    pub(crate) force: bool,
    pub(crate) dry_run: bool,
    pub(crate) depth: Option<i32>,
}


impl Methods {
    pub(crate) fn set_upstream(&self, repo: &GitUtils, branch_name: &str, remote_name: &str) -> Result<(), Error> {
        match self {
            Methods::Upstream => {
                let mut conf = repo.config()?;

                let merge_ref = format!("refs/heads/{}", branch_name);

                conf.set_str(&format!("branch.{}.remote", branch_name), remote_name)?;
                conf.set_str(&format!("branch.{}.merge", branch_name), &merge_ref)?;
                Ok(())
            }
            _ => { panic!("Not support method"); }
        }
    }
}
