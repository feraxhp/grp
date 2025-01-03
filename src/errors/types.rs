// Copyright 2024 feraxhp
// Licensed under the MIT License;

use color_print::cformat;
use crate::errors::error::Error;

#[derive(Clone)]
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
    /// Needs a vector of length N
    /// - 0: owner/repo
    /// - 1..N: Additional information
    NotFoundRepo,
    /// Needs a vector of length 1
    /// - 0: Error message
    /// - 1: Object
    Dezerialized,
    /// Needs a vector of length 1
    /// - 0: name of the unimplemented feature
    Unimplemented,
}

impl ErrorType {
    pub(crate) fn get_message(&self) -> String {
        match self {
            ErrorType::Unknown => "Unknown error".to_string(),
            ErrorType::Unauthorized => "Bad credentials".to_string(),
            ErrorType::AlreadyExists => "Repository already exists".to_string(),
            ErrorType::BadTokenScope => "Bad token scope".to_string(),
            ErrorType::NotFound => "User/org does not exist".to_string(),
            ErrorType::NotFoundRepo => "Repository not found".to_string(),
            ErrorType::Dezerialized => "Error deserializing".to_string(),
            ErrorType::Unimplemented => "Unimplemented".to_string(),
        }
    }

    pub(crate) fn get_content(&self, vec: Vec<&str>) -> Vec<String> {
        match self {
            ErrorType::Unknown => vec.iter().map(|s| s.to_string()).collect(),
            ErrorType::Unauthorized => {
                assert_eq!(vec.len(), 2);
                vec![
                    cformat!("<y>* Please check your token.</>"),
                    cformat!("  <g>» Pconf : <m>{}</>", vec[0]),
                    cformat!("  <g>» User  : <m>{}</>", vec[1]),
                ]
            },
            ErrorType::AlreadyExists => {
                assert_eq!(vec.len(), 2);
                vec![
                    cformat!("<y>* Repository already exists!</>"),
                    cformat!("  <g>» Repo: <m>({}/{})</>", vec[0], vec[1]),
                ]
            },
            ErrorType::BadTokenScope => {
                let mut local_vec: Vec<String> = Vec::new();
                local_vec.push(cformat!("<y>* Please check your token.</>"));
                local_vec.push(cformat!("  <g>» You must add the following scopes: </>"));
                vec[1..].iter().enumerate().for_each(|(i, s)| {
                    local_vec.push(cformat!("    <#e3750e>{}. <m>{}</>", i + 1, s));
                });
                local_vec.push(cformat!("  <g>» Pconf : <m>{}</>", vec[0]));
                local_vec
            },
            ErrorType::NotFound => {
                let mut local_vec = vec![
                    cformat!("<y>* User|org: <m>({})</>", vec[0]),
                ];
                if vec.len() > 1 {
                    local_vec.append(&mut vec[1..].iter().map(|s| s.to_string()).collect());
                }

                local_vec
            },
            ErrorType::NotFoundRepo => {
                let mut local_vec = vec![
                    cformat!("<y>» Repo: <m>({})</>", vec[0]),
                ];
                if vec.len() > 1 {
                    local_vec.append(&mut vec[1..].iter().map(|s| s.to_string()).collect());
                }

                local_vec
            },
            ErrorType::Dezerialized => {
                assert_eq!(vec.len(), 2);
                vec![
                    cformat!("<y>* Error deserializing</>"),
                    cformat!("  You must not see this!"),
                    cformat!("  Please report this issue"),
                    cformat!("  Thank you!"),
                    cformat!("* Error message: <#e3750e>{}</>", vec[0]),
                    cformat!("* Object: <#e3750e>{}</>", vec[1])
                ]
            },
            ErrorType::Unimplemented => {
                vec![
                    cformat!("* Error: <#e3750e>{}</>", self.get_message()),
                    cformat!("  <y,i>{}</> is not implemented yet", vec[0]),
                ]
            },
        }
    }
}

pub(crate) fn unimplemented(feature: String) -> Error {
    Error::new(ErrorType::Unimplemented, vec![feature.as_str()])
}