use serde::{Deserialize, Serialize};
use crate::models::gallery::GalleryModel;
use super::domain_types::UnixUtcDateTime;

/// A gallery.
/// 
/// ### Note
/// Currently this is just an alias to `GalleryModel`, as they are essentially 1:1 (barring replacing `NaiveDateTime` with `UnixUtcDateTime`).
/// If they happen to differ somehow later on, this will be split into its own domain type.
pub type Gallery = GalleryModel;

/// Useful statistics to know about the gallery.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GalleryStats {
    pub total_sessions: u32,
    pub total_mercari_items: u32,
    pub latest_scrape: UnixUtcDateTime
}