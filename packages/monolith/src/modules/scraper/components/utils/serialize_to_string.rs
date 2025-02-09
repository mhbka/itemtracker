//! Contains any random utility fns for this module.
use serde::{Serialize, Serializer};

/// Seralizes `T` as a string first so that `serde_urlencoded` doesn't error due to nested structs.
pub fn serialize_to_string<T, S>(
    criteria: &T, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer
{
    let json_string = serde_json::to_string(criteria)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}