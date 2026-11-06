use std::fmt::Debug;

use crate::commands::validations::structure::Validations;
use crate::commands::validations::PconfOr;
use crate::usettings::structs::Pconf;


impl<T> Validations for PconfOr<T>
where
    T: Validations + Clone+ Debug,
{
    type Output = Self;

    fn value_parcer(value: &str) -> Result<Self::Output, String> {
        match value {
            "-" => Ok(Self::None),
            value if value.contains(":") => {
                let val = T::value_parcer(value)?;
                Ok(Self::Second(val))
            }
            value => {
                let val = Pconf::value_parcer(value)?;
                Ok(Self::First(val))
            }
        }
    }

    fn strict_value_parcer(value: &str) -> Result<Self::Output, String> {
        match value {
            "-" => Ok(Self::None),
            value if value.contains(":") => {
                let val = T::strict_value_parcer(value)?;
                Ok(Self::Second(val))
            }
            value => {
                let val = Pconf::strict_value_parcer(value)?;
                Ok(Self::First(val))
            }
        }
    }
}

impl<T> Default for &PconfOr<T>
where
    T: Validations + Clone+ Debug,
{ fn default() -> Self { &PconfOr::None } }

impl<T> Default for PconfOr<T>
where
    T: Validations,
    T: Clone,
    T: Debug,
{ fn default() -> Self { PconfOr::None } }
