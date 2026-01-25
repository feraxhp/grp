use std::fs;
use clap::crate_version;

use crate::update::{check::get_latest_version, structs::Version};
use grp_core::error::structs::Error;


impl Version {
    pub async fn validate_version() -> Result<(bool, Version), Error> {
        let current_version = format!("v{}", crate_version!());
        let version = get_latest_version().await?;
        Ok((current_version >= version.name, version.clone()))
    }
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

                let line = line.to_lowercase();
                let line = line.trim_start_matches("id=").replace("\"", "");

                match line.as_str() {
                    "ubuntu" => ".deb",
                    "fedora" => ".rpm",
                    _ => return base
                }
            },
            "windows" => ".exe",
            _ => return base
        };

        let asset = self.assets.iter()
            .find(|asset| asset.name.contains(search));

        match asset {
            None => base,
            Some(a) => { a.archive.clone() }
        }
    }
}