use components::request_orchestrator::RequestOrchestrator;
use crate::{config::ItemAnalysisConfig, galleries::domain_types::GalleryId, messages::{message_types::item_analysis::ItemAnalysisMessage, ImageClassifierSender, ItemAnalysisReceiver}};

mod msg_handler;
mod components;

/// Module in charge of orchestrating analysis of scraped items.
pub struct ItemAnalysisModule {
    config: ItemAnalysisConfig,
    llm_requester: RequestOrchestrator,
    galleries_in_progress: Vec<GalleryId>,
    msg_receiver: ItemAnalysisReceiver
}

impl ItemAnalysisModule {
    /// Initialize the module.
    pub fn init(
        config: ItemAnalysisConfig, 
        msg_receiver: ItemAnalysisReceiver,
        img_classifier_msg_sender: ImageClassifierSender
    ) -> Self {
        let llm_requester = RequestOrchestrator::new(
            config.clone(),
            img_classifier_msg_sender
        );
        Self { 
            config, 
            galleries_in_progress: vec![],
            llm_requester,
            msg_receiver
        }
    }

    /// Start accepting and handling messages.
    pub async fn run(&mut self) {
        tracing::info!("ItemAnalysisModule is running...");
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