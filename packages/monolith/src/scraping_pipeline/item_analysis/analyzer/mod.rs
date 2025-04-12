use std::collections::HashMap;

use anthropic::AnthropicRequester;
use openai::OpenAIRequester;

use crate::{config::ItemAnalysisConfig, domain::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, item_data::MarketplaceItemData, pipeline_items::MarketplaceAnalyzedItems}};

mod anthropic;
mod openai;

/// Orchestrates requesting of the LLM for a gallery's items.
pub(super) struct Analyzer {
    config: ItemAnalysisConfig,
    anthropic_requester: AnthropicRequester,
    openai_requester: OpenAIRequester
}

impl Analyzer {
    /// Initialize the analyzer.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        let anthropic_requester = AnthropicRequester::new(config.clone());
        let openai_requester = OpenAIRequester::new(config.clone());
        Self { 
            config,
            anthropic_requester,
            openai_requester
        }   
    }

    /// Request analysis of a gallery's items, and sends the items to the next stage.
    pub async fn analyze_gallery(
        &mut self, 
        items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
        eval_criteria: &EvaluationCriteria
    ) -> HashMap<Marketplace, MarketplaceAnalyzedItems> {
        self.anthropic_requester
            .analyze_gallery(items, eval_criteria)
            .await
    }
}