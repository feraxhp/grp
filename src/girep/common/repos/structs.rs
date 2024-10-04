// Copyright 2024 feraxhp
// Licensed under the MIT License;

pub struct DebugData {
    pub rtype: Rtype,
    pub owner: String,
    pub repo: Option<String>,
}

pub enum Rtype {
    List,
    Create,
    Delete,
    UserList,
    ListOrg,
}