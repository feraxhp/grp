use crate::girep::error::structs::Error;

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
