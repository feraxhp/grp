// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{Arg, arg};
use clap::builder::ValueParser;
use color_print::cformat;

use crate::commands::validations::repo::RepoStructure;
use crate::commands::validations::structure::{Raw, Validations};
use crate::usettings::structs::{Pconf, Usettings};
use super::super::completions::structure::Completer;


pub(crate) struct Arguments;

#[allow(dead_code)]
impl Arguments {
    pub(crate) fn pconf(required: bool, plus: bool) -> Arg {
        let value_parser = if plus { Pconf::value_parcer }
        else { Pconf::strict_value_parcer };
        
        arg!([pconf])
            .help(pconf_help(!plus))
            .value_parser(value_parser)
            .required(required)
            .add(Usettings::complete())
    }
    
    pub(crate) fn raw_pconf(required: bool, plus: bool) -> Arg {
        let value_parser = if plus { Raw::<Pconf>::value_parcer }
        else { Raw::<Pconf>::strict_value_parcer };
        
        arg!([pconf])
            .help(pconf_help(!plus))
            .value_parser(value_parser)
            .required(required)
            .add(Usettings::complete())
    }
    
    pub(crate) fn repo_structure(pconf: bool, required: bool) -> Arg {
        let parcer = if !pconf { RepoStructure::value_parcer } 
        else { RepoStructure::strict_value_parcer };
        
        arg!(<repo> "The repository data as [pconf]:<owner>/<repo>")
            .help(repo_structure_help(pconf))
            .value_parser(parcer)
            .required(required)
            .add(RepoStructure::complete())
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

fn pconf_help(strict: bool) -> String { 
    if strict { cformat!("Platform configuration to be use") }
    else      { cformat!("Platform configuration to be use <g,i>(use the default with '.')</>") }
}

fn repo_structure_help(strict: bool) -> String { 
    if strict { cformat!("The repository data as <i><<pconf>>:<<owner>>/<<repo>></>") }
    else      { cformat!("The repository data as <i>[pconf]:<<owner>>/<<repo>></>") }
}
