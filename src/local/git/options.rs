use git2::{Error, Repository};


#[derive(PartialEq, Clone)]
pub(crate) enum Methods {
    ALL,
    TAG(String),
    TAGS,
    DEFAULT,
    BRANCHES,
    UPSTREAM
}

#[derive(Clone)]
pub(crate) struct Options {
    pub(crate) method: Methods,
    pub(crate) remote: Option<String>,
    pub(crate) branch: Option<String>,
    pub(crate) force: bool,
    pub(crate) dry_run: bool
}


impl Methods {
    pub(crate) fn set_upstream(&self, repo: &Repository ,branch_name: &str, remote_name: &str) -> Result<(), Error> {
        match self {
            Methods::UPSTREAM => {
                let mut conf = repo.config()?;

                let merge_ref = format!("refs/heads/{}", &branch_name);

                conf.set_str(&format!("branch.{}.remote", &branch_name), &remote_name)?;
                conf.set_str(&format!("branch.{}.merge", &branch_name), &merge_ref)?;
                Ok(())
            }
            _ => { panic!("Not support method"); }
        }
    }
}