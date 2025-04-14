//! This module contains domain newtypes, mostly for gallery parameters.

use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};
use chrono::{DateTime, TimeZone, Utc};
use croner::{errors::CronError, Cron};
use diesel::deserialize::FromSql;
use diesel::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use uuid::Uuid;

/// All supported marketplaces.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Marketplace {
    Mercari
}

impl Marketplace {
    /// Attempts to convert from a string; return an error if not possible.
    /// 
    /// The strings used should be the exact same as used in the `Display` impl.
    pub fn from_string(string: &str) -> Result<Self, ()> {
        Ok(
            match string {
            "Mercari" => Self::Mercari,
            _ => Err(())?
            }
        )
    }
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
#[derive(Clone, Debug, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
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

// So that we can directly write to/pull from SQL
impl FromSql<Text, Pg> for ValidCronString {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let val: String = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
        Self::new(val)
            .map_err(|e| e.into())
    }
}

// ^^
impl ToSql<Text, Pg> for ValidCronString {
    fn to_sql(&self, out: &mut diesel::serialize::Output<Pg>) -> diesel::serialize::Result {
        <String as ToSql<Text, Pg>>::to_sql(&self.0, &mut out.reborrow())
    }
}

/// A wrapper for a gallery ID.
/// 
/// There is (currently) no special functionality or validation; this exists simply because the gallery ID is a heavily used domain type.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GalleryId(Uuid);

impl Deref for GalleryId {
    type Target = Uuid;
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

impl From<Uuid> for GalleryId {
    fn from(value: Uuid) -> Self {
        Self(value)
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

    /// Instantiate with a given datetime.
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
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