use std::{env, ffi::OsStr};
use clap_complete::CompletionCandidate;

use crate::commands::completions::git::{branch::Branch, remote::Remote};

use super::super::structure::Completer;


pub struct Upstream;

impl<'a> Completer for Upstream {
    fn canditates(current: &OsStr) -> Vec<CompletionCandidate> {
        let len: usize = env::args().len();
        let index: Option<usize> = env::args()
            .rposition(|arg| matches!(arg.as_str(), "-u" | "--set-upstream"))
            .map(|index| index + 1);
        
        match index {
            Some(index) if (len - index) == 0 => vec![CompletionCandidate::new("")],
            Some(index) if (len - index) == 1 => Remote::canditates(current),
            Some(index) if (len - index) == 2 => Branch::canditates(current),
            _ => Vec::with_capacity(0),
        }
    }
}
