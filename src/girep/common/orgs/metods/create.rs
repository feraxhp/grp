// Copyright 2024 feraxhp
// Licensed under the MIT License;

use crate::errors::error::Error;
use crate::girep::common::orgs::org::Org;
use crate::girep::platform::Platform;

impl Platform {
    pub(crate) fn create_org(&self, name: String, endpoint: String) -> Result<Org, Error>{
        todo!()
    }
}