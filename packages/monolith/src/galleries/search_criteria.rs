use serde::{Serialize, Deserialize};

/// The search criteria used for all marketplaces within the gallery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GallerySearchCriteria {
    keyword: String,
    exclude_keyword: String,
    min_price: Option<f32>,
    max_price: Option<f32>,
}