// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::cmp::max;

pub(crate) struct Repo {
    pub(crate) full_name: String,
    pub(crate) description: String, // optional
    pub(crate) state: String, // private, public
    pub(crate) html_url: String, // for creating a repo
    pub(crate) clone_url: String,
}

impl Repo {
    pub(crate) fn clone(&self) -> Repo {
        Repo {
            full_name: self.full_name.clone(),
            description: self.description.clone(),
            state: self.state.clone(),
            html_url: self.html_url.clone(),
            clone_url: self.clone_url.clone(),
        }
    }
}

#[macro_export]
macro_rules! show {
    ($r:expr) => {
        use std::cmp::max;

        if $r.is_empty() {
            eprintln!("No repositories found");
            return;
        }

        let length = $r.len().to_string().len();
        let max_name = $r.iter().map(|r| r.full_name.len()).max().unwrap();
        let max_state = $r.iter().map(|r| r.state.len()).max().unwrap();

        let max_name = max(4, max_name);
        let max_state = max(5, max_state);

        eprintln!(
            " {0:#^dig$} | {1: <width$} | {2: <state$} | {3}",
            "#", "Name", "State", "git clone",
            width = max_name,
            state = max_state,
            dig = length,
        );
        for (index, repo) in $r.iter().enumerate() {
            eprintln!(
                " {0: ^dig$} | {1: <width$} | {2: <state$} | {3}",
                index + 1, repo.full_name, repo.state, repo.clone_url,
                width = max_name,
                state = max_state,
                dig = length,
            );
        }
    };
}