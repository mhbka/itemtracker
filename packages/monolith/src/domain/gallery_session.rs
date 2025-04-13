use super::{domain_types::{GalleryId, UnixUtcDateTime}, pipeline_states::GalleryFinalState};

/// Represents a session of item scraping + processing.
/// 
/// This is always tied to a gallery via the `gallery_id`.
pub struct GallerySession {
    pub id: SessionId,
    pub gallery_id: GalleryId,
    pub created: UnixUtcDateTime,
    pub data: GalleryFinalState
}

/// The type of `GallerySession` ID.
pub type SessionId = i32;