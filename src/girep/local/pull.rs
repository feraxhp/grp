use crate::girep::config::Config;
use crate::girep::local::git_utils::structure::GitUtils;
use crate::girep::platform::Platform;
use color_print::cformat;
use git2::{BranchType, Error, Repository};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub(crate) struct Options {
    pub(crate) remote: Option<String>,
    pub(crate) branch: Option<String>,
    pub(crate) force: bool,
    pub(crate) dry_run: bool
}

impl Platform {
    pub(crate) fn pull_repo(&self,
        path: PathBuf, options: Options, conf: Config
    ) -> Result<Vec<String>, Error> {
        todo!()
    }
}

