// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{command, Command};
use clap::builder::Str;

pub(crate) struct Commands;

impl Commands {
    fn base(name: &'static str, aliases: impl IntoIterator<Item = impl Into<Str>>, about: &'static str) -> Command
    { command!(name).aliases(aliases).about(about) }

    pub(crate) fn list(about: &'static str) -> Command
    { Self::base("list", ["ls"], about) }

    pub(crate) fn create(about: &'static str) -> Command
    { Self::base("create", ["+", "cr", "crt"], about) }

    pub(crate) fn delete(about: &'static str) -> Command
    { Self::base("delete", ["rm", "del"], about) }
}
