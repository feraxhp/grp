// Copyright 2024 feraxhp
// Licensed under the MIT License;

pub(crate) struct Repo {
    pub(crate) full_name: String,
    pub(crate) description: String, // optional
    pub(crate) state: String, // private, public
    pub(crate) html_url: String, // for creating a repo
    pub(crate) clone_url: String,
}

#[macro_export]
macro_rules! show {
    ($r:expr) => {
        let max_name = $r.iter().map(|r| r.full_name.len()).max().unwrap();
        let length = $r.len().to_string().len();

        println!(
            "{0:#^dig$} | {1: <width$} | {2: <7} | {3: <10}",
            "#", "Name", "State", "git clone",
            width = max_name,
            dig = length,
        );
        for (index, repo) in $r.iter().enumerate() {
            println!(
                "{0: ^dig$} | {1: <width$} | {2: <7} | {3: <10}",
                index + 1, repo.full_name, repo.state, repo.clone_url,
                width = max_name,
                dig = length,
            );
        }
    };
}