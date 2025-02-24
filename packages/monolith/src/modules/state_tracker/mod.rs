use inner_state::InnerState;
use crate::{config::state_tracker::StateTrackerConfig, messages::{message_types::state_tracker::StateTrackerMessage, StateTrackerReceiver}};

mod inner_state;

/// This module tracks (and sometimes manages) the state of galleries in the pipeline.
/// 
/// This is useful since we can persist the state to a temporary store like Redis,
/// and continue from the previous state in case of application restarts.
/// 
/// # API
/// The module has the following API.
/// 
/// ## Add
/// Add a gallery to the state. It can be added in any state.
/// 
/// Returns an `Err` if the gallery already exists.
/// 
/// ## Check
/// Check if the gallery exists in state.
/// 
/// ## Check State
/// Check the gallery's state type.
/// 
/// Returns an `Err` if it doesn't exist.
/// 
/// ## Take
/// *Takes* a gallery's data, leaving it stored as `None`.
/// 
/// 
/// 
/// ## Put
/// 
/// ## Update
/// Update a gallery by setting a new state for it.
/// 
/// ## Remove
/// Remove the gallery from the state. It can be removed while in any state.
/// 
/// Returns an `Err` if the gallery doesn't exist.
pub struct StateTrackerModule {
    config: StateTrackerConfig,
    state: InnerState,
    msg_receiver: StateTrackerReceiver
}

impl StateTrackerModule {
    pub fn init(config: StateTrackerConfig, msg_receiver: StateTrackerReceiver) -> Self {
        Self {
            config,
            state: InnerState::init(),
            msg_receiver
        }
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        tracing::info!("StateTrackerModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: StateTrackerMessage) {
        match msg {
            StateTrackerMessage::AddGallery(msg) => {
                msg.act(|(gallery_id, gallery)| {
                    tracing::trace!("Got message to add gallery {gallery_id} to state"); 
                    self.state.add_gallery(gallery_id, gallery)
            }   );
            },
            StateTrackerMessage::CheckGalleryDoesntExist(msg) => {
                msg.act(|gallery_id| {
                    tracing::trace!("Got message to check (non-)existence of gallery {gallery_id} state"); 
                    self.state.check_gallery_doesnt_exist(gallery_id)
                });
            },
            StateTrackerMessage::CheckGalleryState(msg) => {
                    msg.act(|(gallery_id, state_type)| {
                    tracing::trace!("Got message to check gallery {gallery_id} state"); 
                    self.state.check_gallery_state(gallery_id, state_type)
                });
            },
            StateTrackerMessage::TakeGalleryState(msg) => {
                msg.act(|(gallery_id, requested_state_type)| {
                    tracing::trace!("Got message to take gallery {gallery_id} state"); 
                    self.state.take_gallery_state(gallery_id, requested_state_type)
                });
            },
            StateTrackerMessage::UpdateGalleryState(msg) => {
                msg.act(|(gallery_id, updated_state)| {
                    tracing::trace!("Got message to update gallery {gallery_id} from state"); 
                    self.state.update_gallery_state(gallery_id, updated_state)
                });
            },
            StateTrackerMessage::RemoveGallery(msg) => {
                msg.act(|gallery_id| {
                    tracing::trace!("Got message to remove gallery {gallery_id} from state"); 
                    self.state.remove_gallery(gallery_id)
                });
            },
        }
    }
}