use serde::{Deserialize, Serialize, Serializer};

use crate::galleries::{domain_types::{GalleryId, UnixUtcDateTime}, search_criteria::GallerySearchCriteria};

/// The request form sent to the Scrapyd spider for scraping individual items.
/// 
/// TODO: Maybe look into creating a PR to add/extend the nested struct error in `serde_urlencoded` for this... o~o
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SearchScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    #[serde(serialize_with = "serialize_criteria_as_json")]
    pub search_criteria: GallerySearchCriteria,
    pub up_to: UnixUtcDateTime
}

/// Seralizes `GallerySearchCriteria` as a string first so `serde_urlencoded` doesn't error, as it's a nested struct.
fn serialize_criteria_as_json<S>(
    criteria: &GallerySearchCriteria, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let json_string = serde_json::to_string(criteria)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}