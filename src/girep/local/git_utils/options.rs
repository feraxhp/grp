use git2::{Error, Remote, Repository};

#[derive(PartialEq, Clone)]
pub(crate) enum Methods {
    DEFAULT,
    ALL,
    BRANCHES,
    TAGS,
    UPSTREAM
}

pub(crate) struct Options {
    pub(crate) method: Methods,
    pub(crate) remote: Option<String>,
    pub(crate) branch: Option<String>,
    pub(crate) force: bool,
    pub(crate) dry_run: bool
}


impl Methods {
    pub(crate) fn get_refs(&self, branch: &str, force: &str, remote: Option<&Remote>) -> Vec<String> {
        match (self, remote) {
            (Methods::DEFAULT, Some(remote)) => {
                    let refs = remote.refspecs();
                    let mut specs: Vec<String> = Vec::new();
                    for i in refs {
                        if let Some(src) = i.src() {
                            if let Some(dst) = i.dst() {
                                specs.push(format!("{}:{}", src, dst));
                            } else if i.is_force() {
                                specs.push(format!("+{}", src));
                            } else {
                                specs.push(src.to_string());
                            }
                        }
                    }
                    specs
            }
            (Methods::DEFAULT, None) |
            (Methods::UPSTREAM, _) => vec![format!("{}refs/heads/{branch}:refs/heads/{branch}", force, branch = branch)],
            (Methods::ALL, _) => vec![format!("{}refs/*:refs/*", force)],
            (Methods::BRANCHES, _) => vec![format!("{}refs/heads/*:refs/heads/*", force)],
            (Methods::TAGS, _) => vec![format!("{}refs/tags/*:refs/tags/*", force)]
        }
    }

    pub(crate) fn get_ref_specs(&self, branch_name: &str, force: bool, remote: Option<&Remote>) -> Vec<String> {
        let force = if force { "+" } else { "" };
        let mut ref_specs: Vec<String> = vec![];

        let mut refs = self.get_refs(branch_name, force, remote);
        ref_specs.append(&mut refs);
        ref_specs
    }

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