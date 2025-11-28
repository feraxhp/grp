// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{Arg, arg};
use clap::builder::ValueParser;

use crate::commands::validations::repo::RepoStructure;
use crate::commands::validations::structure::Validations;
use crate::girep::usettings::structs::{Pconf, Usettings};
use super::super::completions::structure::Completer;


pub(crate) struct Arguments;

#[allow(dead_code)]
impl Arguments {
    pub(crate) fn pconf(required: bool, plus: bool) -> Arg {
        let value_parser = if plus { Pconf::value_parcer }
        else { Pconf::strict_value_parcer };
        
        arg!([pconf] "Platform configuration to be use")
            .value_parser(value_parser)
            .required(required)
            .add(Usettings::complete())
    }

    pub(crate) fn repo_structure(pconf: bool, required: bool) -> Arg {
        let parcer = if !pconf { RepoStructure::value_parcer } 
        else { RepoStructure::strict_value_parcer };
        
        arg!(<repo> "The repository data as [pconf]:<owner>/<repo>")
            .value_parser(parcer)
            .required(required)
    }

    pub(crate) fn path(required: bool, about: &str) -> Arg {
        Arg::new("path")
            .help(about.to_owned())
            .required(required)
            .value_hint(clap::ValueHint::DirPath)
            .value_parser(ValueParser::path_buf())
    }

    pub(crate) fn path_flag(required: bool, about: &str) -> Arg {
        Arg::new("path")
            .short('p')
            .long("path")
            .help(about.to_owned())
            .required(required)
            .value_hint(clap::ValueHint::DirPath)
            .value_parser(ValueParser::path_buf())
    }
}
