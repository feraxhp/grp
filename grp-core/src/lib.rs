//! # grp-core
//! is a library that contains the abstracted logic to 
//! interact with the most used platforms for git repositories 
//! for know it supports _github_, _gitlab_, _gitea_, _forgejo_ and _codeberg_
//! 
//! ## How it works?
//! grp-core expose an Enum called `Platform` witch allows you 
//! to deside wich platform to interact with, it also provides a self
//! matching method to create the enum based on a configuration.
//! 
//! ### Example 1
//! create a github interactive platform (only allows you to interact with github)
//! 
//! ~~~
//! use grp_core::Platform;
//! 
//! let platform = Platform::Github;
//! ~~~
//! ### Example 2
//! create an interactive platform based on a &str parameter, this allows 
//! you to create one logic for any platform and just change the &str to 
//! change the platform you will interact with.
//! 
//! ~~~
//! use grp_core::Platform;
//! 
//! let platform = Platform::matches("github");
//! 
//! assert!(platform == Platform::Github);
//! ~~~
//! 
//! ## Configuration
//! grp-core expose an struct called `Config`, wich contains the basic information to 
//! connect and authenticate to the platform and its needed almost in every interaction method.
//!  
//! a feature rich example can be found at _https://github.com/feraxhp/grp_
//! 

pub mod animation;
mod platform;
mod specific;
mod request;
mod config;
mod system;
mod common;
mod error;
mod json;

pub use platform::SUPPORTED_REPOS;
pub use platform::Platform;

pub use error::structs::Error;
pub use error::types::ErrorType;

pub mod structs {
    pub use super::common::structs::*;
    pub use super::common::users::structs::*;
}

pub use json::JSON;
pub use config::Config;
