
use std::ffi::OsStr;
use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};


pub(crate) trait Completer {
    fn complete() -> ArgValueCompleter;
    fn canditates(current: &OsStr) -> Vec<CompletionCandidate>;
}
