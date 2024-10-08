// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::errors::error::Error;
use crate::update::metadata::Version;

pub(crate) trait Updater {
    fn update(version: Version) -> Result<(), Error>;
}
