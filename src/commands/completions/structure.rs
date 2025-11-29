
use std::ffi::OsStr;
use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};


pub(crate) trait Completer: 'static {
    fn complete() -> ArgValueCompleter { ArgValueCompleter::new(Self::canditates) }
    fn canditates(current: &OsStr) -> Vec<CompletionCandidate>;
}
