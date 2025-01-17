use serde::{Serialize, Deserialize};

/// The search criteria used for all marketplaces within the gallery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GallerySearchCriteria {
    keyword: String,
    #[serde(rename = "excludeKeyword")] // camelCase is expected from the scraper
    exclude_keyword: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_price: Option<f32>,
}