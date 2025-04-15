use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use crate::models::gallery::GalleryModel;
use super::{domain_types::{GalleryId, Marketplace, UnixUtcDateTime}, pipeline_states::GallerySchedulerState};

/// A gallery.
/// 
/// ### Note
/// Currently this is just an alias to `GalleryModel`, as they are essentially 1:1 (barring replacing `NaiveDateTime` with `UnixUtcDateTime`).
/// If they happen to differ somehow later on, this will be split into its own domain type.
pub type Gallery = GalleryModel;

impl Gallery {
    /// Convert the gallery to the scheduler state.
    pub fn to_scheduler_state(self) -> GallerySchedulerState {
        let marketplace_previous_scraped_datetimes = Marketplace::iter()
            .filter_map(|marketplace| {
                let datetime = match marketplace {
                    Marketplace::Mercari => self.mercari_last_scraped_time
                };
                match datetime {
                    Some(dt) => Some((marketplace, UnixUtcDateTime::new(dt.and_utc()))),
                    None => None
                }
            })
            .collect();

        GallerySchedulerState {
            gallery_id: self.id.into(),
            scraping_periodicity: self.scraping_periodicity,
            search_criteria: self.search_criteria,
            marketplace_previous_scraped_datetimes,
            evaluation_criteria: self.evaluation_criteria,
        }
    }
}

/// Useful statistics about the gallery.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GalleryStats {
    pub total_sessions: u32,
    pub total_mercari_items: u32,
    pub latest_scrape: UnixUtcDateTime
}