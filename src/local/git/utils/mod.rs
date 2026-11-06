pub mod credentials;
pub mod branch;
pub mod remote;
pub mod rebase;
pub mod merge;
pub mod path;
pub mod tags;
pub mod new;


use std::ops::Deref;
use git2::Repository;


pub struct GitUtils {
    git2: Repository
}

impl Deref for GitUtils {
    type Target = Repository;
    fn deref(&self) -> &Self::Target { &self.git2 }
}
