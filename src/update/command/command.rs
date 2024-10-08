// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{command, Command};

pub(crate) fn update_command() -> Command{
    command!("update")
        .aliases(["up"])
        .about("Autoupdate grp if the system OS is supported")
}
