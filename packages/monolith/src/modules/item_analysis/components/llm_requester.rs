use futures::future::{join, join_all};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{config::ItemAnalysisConfig, galleries::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, scraping_pipeline::GalleryScrapedState}, messages::message_types::item_analysis::ItemAnalysisError};

/// Orchestrates requesting of the LLM for a gallery's items.
pub(in crate::modules::item_analysis) struct LLMRequester {
    config: ItemAnalysisConfig,
    request_client: Client,
}

impl LLMRequester {
    /// Initialize the requester.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        Self { 
            config,
            request_client: Client::new()
        }
    }

    /// Carries out requesting the LLM for a gallery.
    pub async fn request_gallery(&mut self, gallery: GalleryScrapedState) -> Result<(), ItemAnalysisError> {
        let gallery_id = gallery.gallery_id;
        let items = gallery.items;
        let eval_criteria_string = gallery.evaluation_criteria.describe_criteria();
        
        let gallery_requests = self.build_requests(items.clone(), eval_criteria_string);
        for (marketplace, requests) in gallery_requests {
            
        }
        
        Ok(())
    }
 
}

/// The request form for querying Anthropic API.
/// 
/// **NOTE**: There are other optional parameters, but they're left out as we don't (currently) use them.
/// Check the docs for what they are.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicRequestForm {
    model: String,
    max_tokens: usize,
    messages: Vec<AnthropicMessage>,
    system: String
}

/// A single message in the content of a Anthropic API request.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicMessage {
    role: String,
    content: String
}

/// The response received from a Anthropic API request.
/// 
/// **NOTE**: There is other data returned in the response, but we don't (currently) use it,
/// so we just leave them out. Check the docs for what they are.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicResponse {
    id: String,
    content: Vec<AnthropicResponseContent>,
    usage: AnthropicUsage
}

/// The actual response from the LLM.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicResponseContent {
    text: String,
    #[serde(alias = "type")] // can't directly name a struct member `type` as it's a keyword
    content_type: String
}

/// The usage data for this query.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicUsage {
    input_tokens: usize,
    output_tokens: usize
}
