use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Possible errors emitted by the scheduler.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum SchedulerError {
    #[error("The gallery is already in the scheduler")]
    GalleryAlreadyExists,
    #[error("The gallery doesn't exist")]
    GalleryDoesntExist,
    #[error("The ID within the updated gallery doesn't match the gallery ID")]
    UpdatedGalleryDoesntMatch,
    #[error("Failed to send/receive a message to/from the scheduler")]
    MessageFailure,
    #[error("Encountered an uncategorised error: {message}")]
    Other { message: String },
}
