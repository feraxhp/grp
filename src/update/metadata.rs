// Copyright 2024 feraxhp
// Licensed under the MIT License;

use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) struct Version {
    pub name: String,
    pub assets: Vec<Asset>
}

#[derive(Deserialize, Clone)]
pub(crate) struct Asset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub archive: String
}

impl Version {
    pub(crate) fn get_os_url(&self) -> String {
        let base = format!("https://github.com/feraxhp/grp/releases/tag/{}", self.name);
        let search = match std::env::consts::OS {
            "linux" => {
                let content = fs::read_to_string("/etc/os-release").expect("Error");

                let line = match content.lines()
                    .find(|line| { line.to_lowercase().starts_with("id=") }) {
                    Some(line) => line,
                    None => return base
                };

                let line = line.clone().to_lowercase();
                let line = line.trim_start_matches("id=").replace("\"", "");

                match line.as_str() {
                    "ubuntu" => ".deb",
                    "fedora" => ".rpm",
                    os => return base
                }
            },
            "windows" => ".exe",
            os => return base
        };

        let asset = self.assets.iter()
            .find(|asset| asset.name.contains(search));

        match asset {
            None => base,
            Some(a) => { a.archive.clone() }
        }
    }
}
