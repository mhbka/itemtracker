use crate::{config::ImageAnalysisConfig, messages::{message_types::img_analysis::ImgAnalysisMessage, ImgAnalysisReceiver}};

mod msg_handler;

/// Module in charge of orchestrating analysis of scraped items.
pub struct ImageAnalysisModule {
    msg_receiver: ImgAnalysisReceiver
}

impl ImageAnalysisModule {
    /// Initialize the module.
    pub fn init(
        config: ImageAnalysisConfig, 
        msg_receiver: ImgAnalysisReceiver) 
        -> Self {
        Self { msg_receiver }
    }

    /// Start accepting and handling messages.
    pub async fn run(&mut self) {
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ImgAnalysisMessage) {
        match msg {
            ImgAnalysisMessage::StartAnalysis(msg) => {
                msg_handler::handle_start_analysis_msg(msg, self).await;
            },
        }
    }
}