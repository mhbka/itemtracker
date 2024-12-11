use serde::{Deserialize, Serialize};

use crate::galleries::domain_types::{GalleryId, ItemId};

/// The request form sent to the Scrapyd spider for scraping individual items.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct ItemScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    pub item_ids: Vec<ItemId>
}