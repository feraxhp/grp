// Copyright 2024 feraxhp
// Licensed under the MIT License;

use clap::{arg, Arg};
use crate::macros::validations::pconfs::{valid_pconfs, valid_pconfs_and_plus};
use crate::macros::validations::repo::{validate_repo_structure, validate_repo_structure_with_pconf};

pub(crate) struct Arguments;

impl Arguments {
    pub(crate) fn pconf(required: bool, plus: bool) -> Arg {
        let possible_values = match plus {
            true => valid_pconfs_and_plus,
            false => valid_pconfs
        };

        arg!([pconf] "Platform configuration to be use")
            .value_parser(possible_values)
            .required(required)
    }

    pub(crate) fn repo_structure(pconf: bool, required: bool) -> Arg {
        let repo_validation = match pconf {
            true => validate_repo_structure,
            false => validate_repo_structure_with_pconf
        };

        arg!(<repo> "The repository data as [pconf]:<owner>/<repo>")
            .value_parser(repo_validation)
            .required(required)
    }
}