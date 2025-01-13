use components::request_orchestrator::RequestOrchestrator;
use crate::{config::ItemAnalysisConfig, galleries::domain_types::GalleryId, messages::{message_types::item_analysis::ItemAnalysisMessage, ItemAnalysisReceiver}};

mod msg_handler;
mod components;

/// Module in charge of orchestrating analysis of scraped items.
pub struct ItemAnalysisModule {
    config: ItemAnalysisConfig,
    msg_receiver: ItemAnalysisReceiver,
    llm_requester: RequestOrchestrator,
    galleries_in_progress: Vec<GalleryId>
}

impl ItemAnalysisModule {
    /// Initialize the module.
    pub fn init(
        config: ItemAnalysisConfig, 
        msg_receiver: ItemAnalysisReceiver
    ) -> Self {
        let llm_requester = RequestOrchestrator::new(config.clone());
        Self { 
            config, 
            msg_receiver,
            galleries_in_progress: vec![],
            llm_requester
        }
    }

    /// Start accepting and handling messages.
    pub async fn run(&mut self) {
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ItemAnalysisMessage) {
        match msg {
            ItemAnalysisMessage::StartAnalysis(msg) => {
                msg_handler::handle_start_analysis_msg(msg, self).await;
            },
        }
    }
}