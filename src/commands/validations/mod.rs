pub mod structure;
pub mod pconf_or;
pub mod or_exit;
pub mod issues;
pub mod branch;
pub mod range;
pub mod pconf;
pub mod repo;

use std::fmt::Debug;

use crate::usettings::structs::Pconf;
use self::structure::Validations;

pub type PconfOr<T> = DoubleOption<Pconf, T>;

#[derive(Clone, Debug)]
pub enum DoubleOption<F, S>
where 
    F: Validations + Clone + Debug,
    S: Validations + Clone + Debug,
{
    None,
    First(F::Output),
    Second(S::Output),
}
