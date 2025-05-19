use thiserror::Error;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::error::SendError, oneshot::error::RecvError};

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

impl From<RecvError> for SchedulerError {
    fn from(value: RecvError) -> Self {
        SchedulerError::MessageFailure
    }
}

impl<T> From<SendError<T>> for SchedulerError {
    fn from(value: SendError<T>) -> Self {
        SchedulerError::MessageFailure
    }
}