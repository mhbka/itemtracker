use futures::future::{join, join_all};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{config::ItemAnalysisConfig, galleries::pipeline_states::GalleryScrapedState, messages::message_types::item_analysis::ItemAnalysisError};

use super::anthropic::AnthropicRequester;

/// Orchestrates requesting of the LLM for a gallery's items.
pub(in crate::modules::item_analysis) struct RequestOrchestrator {
    config: ItemAnalysisConfig,
    anthropic_requester: AnthropicRequester
}

impl RequestOrchestrator {
    /// Initialize the requester.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        let anthropic_requester = AnthropicRequester::new(config.clone());
        Self { 
            config,
            anthropic_requester
        }
    }

    /// Carries out requesting the LLM for a gallery.
    pub async fn request_gallery(&mut self, gallery: GalleryScrapedState) -> Result<(), ItemAnalysisError> {
        let analyzed_gallery = self.anthropic_requester
            .analyze_gallery(gallery)
            .await;
        Ok(())
    }
}