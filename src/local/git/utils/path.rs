use std::path::Path;

use git2::Repository;
use mynox::Error;
use mynox::error;

use super::super::structs::GitUtils;


impl GitUtils {
    pub fn get_working_dir<'a>(repo: &'a Repository) -> Result<&'a Path, Error> {
        match repo.workdir() {
            Some(w) => Ok(w),
            None => {
                return Err(error!(
                    "local::git::bare_repository",
                    "Cannot read PR templates from a bare repository",
                    "A working tree is required to find template files"
                ));
            }
        }
    }
}
