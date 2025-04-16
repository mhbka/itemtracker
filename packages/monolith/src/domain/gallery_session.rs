use serde::{Deserialize, Serialize};
use super::{domain_types::{GalleryId, UnixUtcDateTime}, eval_criteria::EvaluationCriteria, pipeline_items::EmbeddedMarketplaceItemWithoutEmbeddings};

/// Represents a session of item scraping + processing.
/// 
/// This is always tied to a gallery via the `gallery_id`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GallerySession {
    pub id: SessionId,
    pub gallery_id: GalleryId,
    pub created: UnixUtcDateTime,
    pub used_evaluation_criteria: EvaluationCriteria,
    pub mercari_items: Vec<EmbeddedMarketplaceItemWithoutEmbeddings>
    
    /* 
    // As we currently only store embedded items, other data required here cannot be pulled from storage,
    // so we only keep the embedded items for Mercari (as above).
    // Once we're able to store all data in here, we can directly use this as a field.
    pub items: GalleryFinalState
    */
}

/// Useful statistics about the gallery.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GallerySessionStats {
    pub created: UnixUtcDateTime,
    pub used_evaluation_criteria: EvaluationCriteria,
    pub total_items: u32
}

/// The type of `GallerySession` ID.
pub type SessionId = i32;