mod forgejo;
mod github;
mod gitlab;
mod gitea;

use std::path::PathBuf;
use std::fs;
use git2::Repository;
use rand::RngExt;

pub struct TempRepo {
    path: PathBuf,
    pub repo: Repository,
}

impl TempRepo {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let id: u64 = rng.random();
        let path = std::env::temp_dir().join(format!("grp_template_test_{id}"));
        fs::create_dir_all(&path).unwrap();
        let repo = Repository::init(&path).unwrap();
        Self { path, repo }
    }

    pub fn new_bare() -> Self {
        let mut rng = rand::rng();
        let id: u64 = rng.random();
        let path = std::env::temp_dir().join(format!("grp_template_bare_test_{id}"));
        fs::create_dir_all(&path).unwrap();
        let repo = Repository::init_bare(&path).unwrap();
        Self { path, repo }
    }

    pub fn write_file(&self, rel_path: &str, content: &str) {
        let full_path = self.path.join(rel_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
