use serde::{Deserialize, Serialize};
use crate::galleries::domain_types::{GalleryId, ItemId};
use crate::modules::scraper::components::utils::serialize_to_string::serialize_to_string;

/// The request form sent to the Scrapyd spider for scraping individual items.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct ItemScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    #[serde(serialize_with = "serialize_to_string")]
    pub item_ids: Vec<ItemId>
}