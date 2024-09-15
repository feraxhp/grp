// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cformat;

pub(crate) enum ErrorType {
    /// N number of strings
    Unknown,
    /// Needs a vector of length 2
    /// - 0: Pconf name
    /// - 1: User
    Unauthorized,
    /// Needs a vector of length 2
    /// - 0: Owner
    /// - 1: Repository
    AlreadyExists,
    /// Needs a vector of length N
    /// - 0: Pconf name
    /// - 1..N: Scopes
    BadTokenScope,
    /// Needs a vector of length N
    /// - 0: User|org
    /// - 1..N: Additional information
    NotFound,
    /// Needs a vector of length 1
    /// - 0: Error message
    Dezerialized,
}

impl ErrorType {
    pub(crate) fn get_message(&self) -> String {
        match self {
            ErrorType::Unknown => "Unknown error".to_string(),
            ErrorType::Unauthorized => "Bad credentials".to_string(),
            ErrorType::AlreadyExists => "Repository already exists".to_string(),
            ErrorType::BadTokenScope => "Bad token scope".to_string(),
            ErrorType::NotFound => "User/org does not exist".to_string(),
            ErrorType::Dezerialized => "Error deserializing".to_string(),
        }
    }

    pub(crate) fn get_content(&self, vec: Vec<&str>) -> Vec<String> {
        match self {
            ErrorType::Unknown => vec.iter().map(|s| s.to_string()).collect(),
            ErrorType::Unauthorized => {
                assert_eq!(vec.len(), 2);
                vec![
                    "* Please check your token.".to_string(),
                    format!("  Pconf name: {}", vec[0]),
                    format!("  User: {}", vec[1]),
                ]
            },
            ErrorType::AlreadyExists => {
                assert_eq!(vec.len(), 2);
                vec![
                    "* Repository already exists".to_string(),
                    cformat!("* Repository: <m>({}/{})</>", vec[0], vec[1]),
                ]
            },
            ErrorType::BadTokenScope => {
                let mut local_vec: Vec<String> = Vec::new();
                local_vec.push("* Please check your token.".to_string());
                local_vec.push("  You must add the following scopes: ".to_string());
                vec[1..].iter().enumerate().for_each(|(i, s)| {
                    local_vec.push(cformat!("    <#e3750e>{}. <m>{}</>", i + 1, s));
                });
                local_vec.push(cformat!("* Pconf name: {}", vec[0]));
                local_vec
            },
            ErrorType::NotFound => {
                let mut local_vec = vec![
                    cformat!("* User|org: <m>({})</>", vec[0]),
                ];
                if vec.len() > 1 {
                    local_vec.append(&mut vec[1..].iter().map(|s| s.to_string()).collect());
                }

                local_vec
            },
            ErrorType::Dezerialized => {
                assert_eq!(vec.len(), 1);
                vec![
                    "* Error deserializing".to_string(),
                    "  You must not see this!".to_string(),
                    "  Please report this issue".to_string(),
                    "  Thank you!".to_string(),
                    cformat!("* Error message: <#e3750e>{}</>", vec[0])
                ]
            },
        }
    }
}
