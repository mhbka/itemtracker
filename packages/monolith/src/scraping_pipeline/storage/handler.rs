use crate::{galleries::{domain_types::GalleryId, pipeline_states::GalleryFinalState}, messages::{message_types::storage::StorageError, StateTrackerSender}};

pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender
}

impl Handler {
    /// Initialize the handler.
    pub fn new(state_tracker_sender: StateTrackerSender) -> Self {
        Self {
            state_tracker_sender
        }
    }

    /// Store a gallery in state.
    pub async fn store_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), StorageError> {
        todo!()
    }

    /// Store a new gallery.
    pub async fn store_gallery(&mut self, gallery: GalleryFinalState) -> Result<(), StorageError> {
        todo!()
    }
}