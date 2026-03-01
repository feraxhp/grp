
use std::ffi::OsStr;
use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};

#[macro_export]
macro_rules! candiates {
    () => { fn canditates(_: &std::ffi::OsStr) -> Vec<String> { unreachable!() } };
}

pub(crate) trait Completer: 'static {
    fn complete() -> ArgValueCompleter { ArgValueCompleter::new(Self::ccanditates) }
    fn ccanditates(current: &OsStr) -> Vec<CompletionCandidate> {
        Self::canditates(current).iter()
            .map(|s| CompletionCandidate::new(s))
            .collect()
    }
    fn canditates(current: &OsStr) -> Vec<String>;
}
