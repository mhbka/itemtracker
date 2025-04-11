//! This module contains domain newtypes, mostly for gallery parameters.

use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};
use chrono::{DateTime, TimeZone, Utc};
use croner::{errors::CronError, Cron};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

/// All supported marketplaces.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Marketplace {
    Mercari
}

impl Display for Marketplace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Marketplace::Mercari => write!(f, "Mercari")
        }
    }
}

/// A String wrapper signifying that it is a valid Cron pattern.
/// 
/// This is used over a `Cron`, as:
/// - `Cron` is not `(De)Serialize`
/// - `Cron` doesn't verify that its string is a valid Cron pattern
#[derive(Clone, Debug, Serialize)]
pub struct ValidCronString(String);

impl ValidCronString {
    /// Instantiate, checking if the string is a valid Cron pattern.
    pub fn new(str: String) -> Result<Self, CronError> {
        match Cron::new(&str).parse() {
            Ok(_) => Ok(Self(str)),
            Err(err) => Err(err)
        }
    } 

    /// Get a (guaranteed valid) `Cron` from the string.
    pub fn get_cron(&mut self) -> Cron {
        Cron::new(&self.0)
            .parse()
            .expect("Should be valid as we've already checked during instantiation")
    }

    /// Get a copy of the string.
    pub fn get_str(&self) -> &str {
        &self.0
    }
}

// Custom implementation to check Cron validity before deserializing.
impl<'de> Deserialize<'de> for ValidCronString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        ValidCronString::new(s).map_err(|_| D::Error::custom("String is not a valid Cron pattern"))
    }
}

/// A String wrapper for a gallery ID.
/// 
/// There is (currently) no special functionality or validation; this exists simply because the gallery ID is a heavily used domain type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct GalleryId(String);

impl Deref for GalleryId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GalleryId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for GalleryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A wrapper for a marketplace item ID.
/// 
/// There is (currently) no special functionality or validation; this exists simply because the item ID is a heavily used domain type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ItemId(String);

impl Deref for ItemId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ItemId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// A DateTime<Utc> wrapper that (de)serializes to/from a UNIX timestamp integer.
/// 
/// There is (currently) no special functionality or validation; this exists simply because UNIX timestamps are easier to work with.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct UnixUtcDateTime(DateTime<Utc>);

impl UnixUtcDateTime {
    /// Instantiate with the current datetime.
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl Deref for UnixUtcDateTime {
    type Target = DateTime<Utc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UnixUtcDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for UnixUtcDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

impl<'de> Deserialize<'de> for UnixUtcDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        let string_timestamp = String::deserialize(deserializer)?;
        let timestamp = string_timestamp
            .parse::<i64>()
            .map_err(|err| serde::de::Error::custom("Could not parse string to i64"))?;
        let datetime = chrono::Utc.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?;
        Ok(UnixUtcDateTime(datetime))
    }
}

impl From<i64> for UnixUtcDateTime {
    fn from(value: i64) -> Self {
        let datetime = chrono::Utc.timestamp_opt(value, 0)
            .single()
            .unwrap_or_else(|| {
                chrono::Utc.timestamp_opt(0, 0)
                .single()
                .unwrap() // NOTE: should be OK as it's just beginning of UNIX time 
            });
        Self(datetime)
    }
}