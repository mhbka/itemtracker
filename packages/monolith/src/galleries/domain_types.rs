//! This module contains domain newtypes, mostly for gallery parameters.

use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};
use croner::{errors::CronError, Cron};
use serde::{Deserialize, Deserializer, Serialize};
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
/// This is used over a `Cron`, as `Cron` is not `(De)Serialize`.
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
            .unwrap() // OK since this newtype checks for validity during instantiation
    }

    /// Get a copy of the string.
    pub fn get_str(&self) -> &str {
        &self.0
    }
}

/// Custom implementation to check Cron validity before deserializing.
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
/// There is (currently) no special functionality or validation; this exist simply because the gallery ID is a heavily used domain type.
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

/// A String wrapper for a marketplace item ID.
/// 
/// There is (currently) no special functionality or validation; this exist simply because the gallery ID is a heavily used domain type.
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
