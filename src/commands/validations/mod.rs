pub mod structure;
pub mod or_exit;
pub mod issues;
pub mod branch;
pub mod range;
pub mod pconf;
pub mod repo;

use std::fmt::Debug;

use crate::usettings::structs::Pconf;
use self::structure::Validations;

#[derive(Clone, Debug)]
pub enum PconfOr<T>
where 
    T: Validations,
    T: Clone,
    T: Debug,
{
    Default,
    Pconf(Pconf),
    Other(T::Output)
}

impl<T> Validations for PconfOr<T>
where
    T: Validations,
    T: Clone,
    T: Debug,
{
    type Output = Self;

    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        match value {
            "-" => Ok(Self::Default),
            value if value.contains(":") => {
                let val = T::value_parcer(value)?;
                Ok(Self::Other(val))
            }
            value => {
                let val = Pconf::value_parcer(value)?;
                Ok(Self::Pconf(val))
            }
        }
    }

    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        match value {
            "-" => Ok(Self::Default),
            value if value.contains(":") => {
                let val = T::strict_value_parcer(value)?;
                Ok(Self::Other(val))
            }
            value => {
                let val = Pconf::strict_value_parcer(value)?;
                Ok(Self::Pconf(val))
            }
        }
    }
}

impl<T> Default for &PconfOr<T>
where
    T: Validations,
    T: Clone,
    T: Debug,
{ fn default() -> Self { &PconfOr::Default } }

impl<T> Default for PconfOr<T>
where
    T: Validations,
    T: Clone,
    T: Debug,
{ fn default() -> Self { PconfOr::Default } }
