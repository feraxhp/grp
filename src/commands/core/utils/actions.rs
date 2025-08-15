use std::{io::{self, Write}, process::exit};

use clap::{builder::{IntoResettable, Resettable}, crate_version, ArgAction};

pub struct ShowVersion;

impl IntoResettable<ArgAction> for ShowVersion {
    fn into_resettable(self) -> Resettable<ArgAction> {
        let version = crate_version!();
        let _ = io::stdout().write(version.as_bytes());
        let _ = io::stdout().flush();
        println!();
        exit(0)
    }
}