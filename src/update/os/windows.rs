// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::girep::errors::error::Error;
use crate::update::metadata::Version;
use crate::update::os::base::Updater;

pub(crate) struct Windows;

impl Updater for Windows{
    fn update(version: Version) -> Result<(), Error> {
        todo!()
    }
}
