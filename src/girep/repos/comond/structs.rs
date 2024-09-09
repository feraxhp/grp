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
}

impl Rtype {
    pub fn to_string(&self) -> String {
        match self {
            Rtype::List => "List".to_string(),
            Rtype::Create => "Create".to_string(),
            Rtype::Delete => "Delete".to_string(),
        }
    }
}