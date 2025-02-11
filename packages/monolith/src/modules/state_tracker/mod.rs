use crate::{config::state_tracker::StateTrackerConfig, messages::{message_types::state_tracker::StateTrackerMessage, StateTrackerReceiver}};

/// This module tracks (and sometimes manages) the state of galleries in the pipeline.
pub struct StateTrackerModule {
    config: StateTrackerConfig,
    msg_receiver: StateTrackerReceiver
}

impl StateTrackerModule {
    pub fn init(config: StateTrackerConfig, msg_receiver: StateTrackerReceiver) -> Self {
        Self {
            config,
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
            
        }
    }
}