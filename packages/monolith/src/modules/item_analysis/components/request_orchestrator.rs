use crate::{config::ItemAnalysisConfig, galleries::pipeline_states::GalleryScrapedState, messages::{message_types::{img_classifier::{ImgClassifierMessage, StartClassificationJob}, item_analysis::ItemAnalysisError}, ImageClassifierSender}};
use super::anthropic::AnthropicRequester;

/// Orchestrates requesting of the LLM for a gallery's items.
pub(in crate::modules::item_analysis) struct RequestOrchestrator {
    config: ItemAnalysisConfig,
    anthropic_requester: AnthropicRequester,
    img_classifier_msg_sender: ImageClassifierSender
}

impl RequestOrchestrator {
    /// Initialize the requester.
    pub fn new(config: ItemAnalysisConfig, img_classifier_msg_sender: ImageClassifierSender) -> Self {
        let anthropic_requester = AnthropicRequester::new(config.clone());
        Self { 
            config,
            anthropic_requester,
            img_classifier_msg_sender
        }
    }

    /// Request analysis of a gallery's items, and sends the items to the next stage.
    pub async fn handle_gallery(
        &mut self, 
        gallery: GalleryScrapedState,
    ) -> Result<(), ItemAnalysisError> {
        let gallery_id = gallery.gallery_id.clone();
        let analyzed_gallery = self.anthropic_requester
            .analyze_gallery(gallery)
            .await;
        let msg = StartClassificationJob { gallery: analyzed_gallery };
        match self.img_classifier_msg_sender
            .send(ImgClassifierMessage::StartClassification(msg))
            .await {
                Ok(_) => {
                    tracing::info!("Successfully sent analyzed gallery items for gallery {gallery_id} to image classifier module");
                }
                Err(err) => {
                    tracing::error!("Error while sending analyzed gallery items to image classifier: {err}")
                },
            }
        Ok(())
    }
}