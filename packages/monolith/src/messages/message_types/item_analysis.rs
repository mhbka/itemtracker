use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::galleries::{domain_types::GalleryId, pipeline_states::GalleryItemAnalysisState};

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemAnalysisError {
}

/// The types of messages the item analysis module can take.
#[derive(Debug)]
pub enum ItemAnalysisMessage {
    /// Message for starting analysis of a gallery in state.
    /// If the gallery isn't in state, an error is logged and nothing happens.
    AnalyzeGallery { gallery_id: GalleryId },
    /// Message for starting analysis of a new gallery.
    AnalyzeGalleryNew { gallery: GalleryItemAnalysisState },
}