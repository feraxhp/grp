use super::error::structs::Error;

/// default implementation for deserialize a json, returning a _grp_core::Error_ directly, 
/// very usefull for use _?_ in an already returning _Result<T, Error>_
/// 
/// # Example
/// ~~~
/// use serde::Deserialize;
/// use grp_core::JSON;
/// 
/// #[derive(Deserialize, Clone, Debug, PartialEq)]
/// pub(crate) struct Version {
///     pub name: String,
///     pub version: String,
/// }
/// 
/// let version: Version = JSON::from_str(&"{\"name\":\"grp\",\"version\": \"v1.0.2\"}").unwrap();
/// 
/// assert_eq!(version, Version{name: "grp".to_string(), version: "v1.0.2".to_string()})
/// ~~~
/// 
pub struct JSON;

impl JSON {
    pub fn from_str<T, S>(text: &S) -> Result<T, Error> 
    where 
        T: serde::de::DeserializeOwned,
        S: AsRef<str>,
    {
        let text = text.as_ref();
        serde_json::from_str(text).map_err(Error::from_serde(&text.to_string()))
    }
}
