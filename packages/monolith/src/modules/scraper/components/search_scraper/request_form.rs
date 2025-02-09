use serde::{Deserialize, Serialize};
use crate::galleries::{domain_types::{GalleryId, UnixUtcDateTime}, search_criteria::GallerySearchCriteria};
use crate::modules::scraper::components::utils::serialize_to_string::serialize_to_string;

/// The request form sent to the Scrapyd spider for scraping individual items.
/// 
/// TODO: Maybe look into creating a PR to add/extend the nested struct error in `serde_urlencoded` for this... o~o
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SearchScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    #[serde(serialize_with = "serialize_to_string")]
    pub search_criteria: GallerySearchCriteria,
    pub up_to: UnixUtcDateTime
}

